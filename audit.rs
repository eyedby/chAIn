// src/audit.rs
use crate::ebpf;
use serde::{Serialize, Deserialize};

// Fixed baseline coefficient for local packet mitigation energy calculation
const ENERGY_PER_PACKET: f64 = 0.0000003; 

#[derive(Serialize, Deserialize, Debug)]
pub struct EfficiencyStats {
    pub blocked_count: u64,
    pub kwh_reclaimed: f64,
    pub status: &'static str,
}

/// Dynamic Applet Trigger: Only evaluates if the card payload is actively loaded in the box state
pub fn get_efficiency_metrics(applet_card_active: bool) -> Option<EfficiencyStats> {
    if !applet_card_active {
        // Applet card is not dropped into the local box slot; return None to bypass execution paths entirely
        return None;
    }

    let blocked_packets = ebpf::get_blocked_count();
    let energy_saved_kwh = blocked_packets as f64 * ENERGY_PER_PACKET;
    
    Some(EfficiencyStats {
        blocked_count: blocked_packets,
        kwh_reclaimed: energy_saved_kwh,
        status: "CARD_ACTIVE",
    })
}
