use crate::ecosystems::PackageMetadata;

pub fn calculate(metadata: &PackageMetadata) -> u8 {
    if let Some(date) = &metadata.latest_release_date {
        // Simplified logic: if released in 2026 or 2025 -> good score
        if date.starts_with("2026") || date.starts_with("2025") || date.starts_with("2024") {
            return 90;
        } else if date.starts_with("2023") {
            return 60;
        } else {
            return 20;
        }
    }
    50
}
