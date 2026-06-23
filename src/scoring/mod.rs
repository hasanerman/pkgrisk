pub mod bus_factor;
pub mod community;
pub mod dependency_tree;
pub mod license;
pub mod maintenance;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryScore {
    pub score: u8,
    pub details: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthScore {
    pub total: u8,
    pub maintenance: CategoryScore,
    pub bus_factor: CategoryScore,
    pub community: CategoryScore,
    pub license: CategoryScore,
    pub dependencies: CategoryScore,
    pub adoption: CategoryScore,
}

pub fn calculate_total_score(
    maintenance: u8,
    bus_factor: u8,
    community: u8,
    adoption: u8,
    license: u8,
    dependencies: u8,
) -> u8 {
    let m = maintenance as f32 * 0.25;
    let b = bus_factor as f32 * 0.20;
    let c = community as f32 * 0.15;
    let a = adoption as f32 * 0.15;
    let l = license as f32 * 0.15;
    let d = dependencies as f32 * 0.10;

    (m + b + c + a + l + d).round() as u8
}
