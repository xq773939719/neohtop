use crate::types::ProcessStaticInfo;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};

pub struct AppState {
    pub sys: Mutex<System>,
    pub process_cache: Mutex<HashMap<u32, ProcessStaticInfo>>,
    pub last_network_update: Mutex<(Instant, u64, u64)>,
}

impl AppState {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_all();

        let initial_rx = sys
            .networks()
            .iter()
            .map(|(_, data)| data.total_received())
            .sum();
        let initial_tx = sys
            .networks()
            .iter()
            .map(|(_, data)| data.total_transmitted())
            .sum();

        Self {
            sys: Mutex::new(sys),
            process_cache: Mutex::new(HashMap::new()),
            last_network_update: Mutex::new((Instant::now(), initial_rx, initial_tx)),
        }
    }
}
