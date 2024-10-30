#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{CpuExt, SystemExt, ProcessExt, System, PidExt, ProcessStatus};
use tauri::State;
use std::sync::Mutex;

struct AppState {
    sys: Mutex<System>,
}

#[derive(serde::Serialize)]
struct ProcessInfo {
    pid: u32,
    ppid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
    status: String,
    user: String,
    command: String,
    threads: Option<u32>,
}

#[derive(serde::Serialize)]
struct SystemStats {
    cpu_usage: Vec<f32>,
    memory_total: u64,
    memory_used: u64,
    uptime: u64,
    load_avg: [f64; 3],
}

#[tauri::command]
async fn get_processes(state: State<'_, AppState>) -> Result<Vec<ProcessInfo>, String> {
    let mut sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
    sys.refresh_all();

    Ok(sys.processes()
        .iter()
        .map(|(pid, process)| {
            let threads = if cfg!(target_os = "macos") {
                use std::process::Command;
                Command::new("ps")
                    .args(["-o", "thcount=", "-p", &pid.as_u32().to_string()])
                    .output()
                    .ok()
                    .and_then(|output| {
                        String::from_utf8(output.stdout)
                            .ok()
                            .and_then(|s| s.trim().parse().ok())
                    })
            } else {
                None
            };

            let status = match process.status() {
                ProcessStatus::Run => "R",      // Running
                ProcessStatus::Sleep => "S",    // Sleeping
                ProcessStatus::Idle => "I",     // Idle
                ProcessStatus::Zombie => "Z",   // Zombie
                ProcessStatus::Stop => "T",     // Stopped
                ProcessStatus::Dead => "X",     // Dead
                _ => "Unknown",
            };

            ProcessInfo {
                pid: pid.as_u32(),
                ppid: process.parent().unwrap_or(sysinfo::Pid::from(0)).as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                status: status.to_string(),
                user: process.user_id()
                    .map(|uid| uid.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                command: process.cmd().join(" "),
                threads,
            }
        })
        .collect())
}

#[tauri::command]
async fn get_system_stats(state: State<'_, AppState>) -> Result<SystemStats, String> {
    let mut sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
    sys.refresh_all();

    let load_avg = sys.load_average();
    Ok(SystemStats {
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        memory_total: sys.total_memory(),
        memory_used: sys.used_memory(),
        uptime: sys.uptime(),
        load_avg: [load_avg.one, load_avg.five, load_avg.fifteen],
    })
}

#[tauri::command]
async fn kill_process(pid: u32, state: State<'_, AppState>) -> Result<bool, String> {
    let sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
    if let Some(process) = sys.process(sysinfo::Pid::from(pid as usize)) {
        Ok(process.kill())
    } else {
        Ok(false)
    }
}

#[tauri::command]
async fn process_stop(pid: u32, state: State<'_, AppState>) -> Result<bool, String> {
    let sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
    if let Some(_process) = sys.process(sysinfo::Pid::from(pid as usize)) {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            Command::new("kill")
                .args(["-STOP", &pid.to_string()])
                .status()
                .map_err(|e| e.to_string())?;
            Ok(true)
        }
        #[cfg(not(target_os = "macos"))]
        {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

#[tauri::command]
async fn process_continue(pid: u32, state: State<'_, AppState>) -> Result<bool, String> {
    let sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
    if let Some(_process) = sys.process(sysinfo::Pid::from(pid as usize)) {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            Command::new("kill")
                .args(["-CONT", &pid.to_string()])
                .status()
                .map_err(|e| e.to_string())?;
            Ok(true)
        }
        #[cfg(not(target_os = "macos"))]
        {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            sys: Mutex::new(System::new_all()),
        })
        .invoke_handler(tauri::generate_handler![
            get_processes,
            get_system_stats,
            kill_process,
            process_stop,
            process_continue
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}