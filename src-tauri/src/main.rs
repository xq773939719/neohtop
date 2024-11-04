#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{CpuExt, SystemExt, ProcessExt, System, PidExt, ProcessStatus};
use tauri::State;
use std::sync::Mutex;
use users::{Users, UsersCache};

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
    memory_free:u64,
    memory_cached: u64,
    uptime: u64,
    load_avg: [f64; 3],
}

#[tauri::command]
async fn get_processes(state: State<'_, AppState>) -> Result<Vec<ProcessInfo>, String> {
    let mut sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
    sys.refresh_all();

    let users_cache = UsersCache::new();

    Ok(sys.processes()
        .iter()
        .map(|(pid, process)| {
            let status = match process.status() {
                ProcessStatus::Run => "Running",
                ProcessStatus::Sleep => "Sleeping",
                ProcessStatus::Idle => "Idle",
                ProcessStatus::Stop => "Stopped",
                ProcessStatus::Zombie => "Zombie",
                ProcessStatus::Tracing => "Tracing",
                ProcessStatus::Dead => "Dead",
                ProcessStatus::Wakekill => "Wake Kill",
                ProcessStatus::Waking => "Waking",
                ProcessStatus::Parked => "Parked",
                ProcessStatus::LockBlocked => "Lock Blocked",
                ProcessStatus::UninterruptibleDiskSleep => "Disk Sleep",
                ProcessStatus::Unknown(_) => "Unknown"
            };

            let user = process.user_id()
                .and_then(|uid| {
                    let uid_num = uid.to_string().parse::<u32>().unwrap_or(0);
                    users_cache.get_user_by_uid(uid_num)
                        .map(|user| format!("{} ({})", user.name().to_string_lossy(), uid_num))
                })
                .unwrap_or_else(|| "-".to_string());

            ProcessInfo {
                pid: pid.as_u32(),
                ppid: process.parent().unwrap_or(sysinfo::Pid::from(0)).as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                status: status.to_string(),
                user,
                command: process.cmd().join(" "),
                threads: None,
            }
        })
        .collect())
}

#[tauri::command]
async fn get_system_stats(state: State<'_, AppState>) -> Result<SystemStats, String> {
    let mut sys = state.sys.lock().map_err(|_| "Failed to lock system state")?;
    sys.refresh_all();

    let memory_cached = sys.available_memory() - sys.free_memory();

    let load_avg = sys.load_average();
    Ok(SystemStats {
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        memory_total: sys.total_memory(),
        memory_used: sys.used_memory(),
        memory_free: sys.available_memory(),
        memory_cached,//FIXME: get accurate value
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

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            sys: Mutex::new(System::new_all()),
        })
        .invoke_handler(tauri::generate_handler![
            get_processes,
            get_system_stats,
            kill_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}