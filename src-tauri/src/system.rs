use crate::state::AppState;
use crate::types::SystemStats;
use std::path::Path;
use std::time::Instant;
use sysinfo::{CpuExt, Disk, DiskExt, NetworkExt, NetworksExt, SystemExt};

#[cfg(not(target_os = "windows"))]
pub fn filter_disks(disks: &[Disk]) -> Vec<&sysinfo::Disk> {
    disks
        .iter()
        .filter(|disk| {
            // Filter for physical disks - typically those mounted at "/"
            disk.mount_point() == Path::new("/")
        })
        .collect()
}

#[cfg(target_os = "windows")]
pub fn filter_disks(disks: &[Disk]) -> Vec<&sysinfo::Disk> {
    disks.iter().collect()
}

fn calculate_network_stats(
    sys: &sysinfo::System,
    last_update: &mut (Instant, u64, u64),
    elapsed: f64,
) -> (u64, u64) {
    let current_rx: u64 = sys
        .networks()
        .iter()
        .map(|(_, data)| data.total_received())
        .sum();
    let current_tx: u64 = sys
        .networks()
        .iter()
        .map(|(_, data)| data.total_transmitted())
        .sum();

    let rx_rate = ((current_rx - last_update.1) as f64 / elapsed) as u64;
    let tx_rate = ((current_tx - last_update.2) as f64 / elapsed) as u64;

    last_update.0 = Instant::now();
    last_update.1 = current_rx;
    last_update.2 = current_tx;

    (rx_rate, tx_rate)
}

fn calculate_disk_stats(sys: &sysinfo::System) -> (u64, u64, u64) {
    let disks = filter_disks(sys.disks());
    let total: u64 = disks.iter().map(|disk| disk.total_space()).sum();
    let used: u64 = disks
        .iter()
        .map(|disk| disk.total_space() - disk.available_space())
        .sum();
    let free: u64 = disks.iter().map(|disk| disk.available_space()).sum();
    (total, used, free)
}

pub fn collect_system_stats(
    sys: &mut sysinfo::System,
    state: &AppState,
) -> Result<SystemStats, String> {
    let mut last_update = state
        .last_network_update
        .lock()
        .map_err(|e| format!("Failed to lock network state: {}", e))?;

    let elapsed = last_update.0.elapsed().as_secs_f64();

    let (network_rx, network_tx) = calculate_network_stats(sys, &mut last_update, elapsed);
    let (disk_total, disk_used, disk_free) = calculate_disk_stats(sys);

    Ok(SystemStats {
        cpu_usage: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        memory_total: sys.total_memory(),
        memory_used: sys.used_memory(),
        memory_free: sys.total_memory() - sys.used_memory(),
        memory_cached: sys.total_memory()
            - (sys.used_memory() + (sys.total_memory() - sys.used_memory())),
        uptime: sys.uptime(),
        load_avg: [
            sys.load_average().one,
            sys.load_average().five,
            sys.load_average().fifteen,
        ],
        network_rx_bytes: network_rx,
        network_tx_bytes: network_tx,
        disk_total_bytes: disk_total,
        disk_used_bytes: disk_used,
        disk_free_bytes: disk_free,
    })
}
