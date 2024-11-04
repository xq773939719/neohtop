#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{CpuExt, SystemExt, ProcessExt, System, PidExt, ProcessStatus};
use tauri::State;
use std::sync::Mutex;
use std::time;
use std::collections::HashMap;
use std::time::Instant;

struct AppState {
    sys: Mutex<System>,
    process_cache: Mutex<HashMap<u32, ProcessStaticInfo>>,
    last_update: Mutex<Instant>,
}

#[derive(Clone)]
struct ProcessStaticInfo {
    name: String,
    command: String,
    user: String,
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
async fn get_processes(state: State<'_, AppState>) -> Result<(Vec<ProcessInfo>, SystemStats), String> {
    let mut sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
    let mut process_cache = state.process_cache.lock().map_err(|_| "Failed to lock process cache")?;
    
    // Batch refresh system information
    sys.refresh_all();

    let processes = sys.processes()
        .iter()
        .map(|(pid, process)| {
            let pid_u32 = pid.as_u32();
            
            // Get or update cache for static process info
            let static_info = process_cache.entry(pid_u32).or_insert_with(|| {
                ProcessStaticInfo {
                    name: process.name().to_string(),
                    command: process.cmd().join(" "),
                    user: process.user_id()
                        .map_or_else(|| "-".to_string(), |uid| uid.to_string()),
                }
            });

            let status = match process.status() {
                ProcessStatus::Run => "Running",
                ProcessStatus::Sleep => "Sleeping",
                ProcessStatus::Idle => "Idle",
                _ => "Unknown"
            };

            ProcessInfo {
                pid: pid_u32,
                ppid: process.parent().unwrap_or(sysinfo::Pid::from(0)).as_u32(),
                name: static_info.name.clone(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                status: status.to_string(),
                user: static_info.user.clone(),
                command: static_info.command.clone(),
                threads: None,
            }
        })
        .collect();

    let load_avg = sys.load_average();
    let system_stats = SystemStats {
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        memory_total: sys.total_memory(),
        memory_used: sys.used_memory(),
        uptime: sys.uptime(),
        load_avg: [load_avg.one, load_avg.five, load_avg.fifteen],
    };

    Ok((processes, system_stats))
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

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            sys: Mutex::new(System::new_all()),
            process_cache: Mutex::new(HashMap::new()),
            last_update: Mutex::new(Instant::now()),
        })
        .invoke_handler(tauri::generate_handler![
            get_processes,
            kill_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}