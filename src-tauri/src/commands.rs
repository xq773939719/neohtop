use crate::state::AppState;
use crate::system::collect_system_stats;
use crate::types::{ProcessInfo, ProcessStaticInfo, SystemStats};
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{PidExt, ProcessExt, ProcessStatus, SystemExt};
use tauri::State;

#[tauri::command]
pub async fn get_processes(
    state: State<'_, AppState>,
) -> Result<(Vec<ProcessInfo>, SystemStats), String> {
    let processes_data;
    let system_stats;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Failed to get system time: {}", e))?
        .as_secs();

    let mut sys = state
        .sys
        .lock()
        .map_err(|e| format!("Failed to lock system state: {}", e))?;

    sys.refresh_all();
    sys.refresh_networks_list();
    sys.refresh_disks_list();

    processes_data = collect_process_data(&sys, current_time);

    system_stats = collect_system_stats(&mut sys, &state)
        .map_err(|e| format!("Failed to collect system stats: {}", e))?;

    let mut process_cache = state
        .process_cache
        .lock()
        .map_err(|e| format!("Failed to lock process cache: {}", e))?;

    let processes = build_process_info(processes_data, &mut process_cache);

    Ok((processes, system_stats))
}

#[tauri::command]
pub async fn kill_process(pid: u32, state: State<'_, AppState>) -> Result<bool, String> {
    let sys = state
        .sys
        .lock()
        .map_err(|e| format!("Failed to lock system state for process termination: {}", e))?;

    if let Some(process) = sys.process(sysinfo::Pid::from(pid as usize)) {
        Ok(process.kill())
    } else {
        Ok(false)
    }
}

// Helper functions
fn collect_process_data(sys: &sysinfo::System, current_time: u64) -> Vec<ProcessData> {
    sys.processes()
        .iter()
        .map(|(pid, process)| {
            let start_time = process.start_time();
            let run_time = if start_time > 0 {
                current_time.saturating_sub(start_time)
            } else {
                0
            };

            ProcessData {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cmd: process.cmd().to_vec(),
                user_id: process.user_id().map(|uid| uid.to_string()),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
                status: process.status(),
                ppid: process.parent().map(|p| p.as_u32()),
                environ: process.environ().to_vec(),
                root: process.root().to_string_lossy().into_owned(),
                virtual_memory: process.virtual_memory(),
                start_time,
                run_time,
                disk_read: process.disk_usage().read_bytes,
                disk_written: process.disk_usage().written_bytes,
                session_id: process.session_id().map(|id| id.as_u32()),
            }
        })
        .collect()
}

// Helper struct for intermediate process data
struct ProcessData {
    pid: u32,
    name: String,
    cmd: Vec<String>,
    user_id: Option<String>,
    cpu_usage: f32,
    memory: u64,
    status: ProcessStatus,
    ppid: Option<u32>,
    environ: Vec<String>,
    root: String,
    virtual_memory: u64,
    start_time: u64,
    run_time: u64,
    disk_read: u64,
    disk_written: u64,
    session_id: Option<u32>,
}

// Helper function to build process info
fn build_process_info(
    processes: Vec<ProcessData>,
    process_cache: &mut std::collections::HashMap<u32, ProcessStaticInfo>,
) -> Vec<ProcessInfo> {
    processes
        .into_iter()
        .map(|data| {
            // Update or get from cache
            let cached_info = process_cache
                .entry(data.pid)
                .or_insert_with(|| ProcessStaticInfo {
                    name: data.name.clone(),
                    command: data.cmd.join(" "),
                    user: data.user_id.clone().unwrap_or_else(|| "-".to_string()),
                });

            let status_str = match data.status {
                ProcessStatus::Run => "Running",
                ProcessStatus::Sleep => "Sleeping",
                ProcessStatus::Idle => "Idle",
                _ => "Unknown",
            };

            ProcessInfo {
                pid: data.pid,
                ppid: data.ppid.unwrap_or(0),
                name: cached_info.name.clone(),
                cpu_usage: data.cpu_usage,
                memory_usage: data.memory,
                status: status_str.to_string(),
                user: cached_info.user.clone(),
                command: cached_info.command.clone(),
                threads: None,
                environ: data.environ,
                root: data.root,
                virtual_memory: data.virtual_memory,
                start_time: data.start_time,
                run_time: data.run_time,
                disk_usage: (data.disk_read, data.disk_written),
                session_id: data.session_id,
            }
        })
        .collect()
}
