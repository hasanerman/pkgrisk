use crate::ecosystems::PackageMetadata;
use crate::config::LicenseConfig;

pub fn calculate(metadata: &PackageMetadata, config: &LicenseConfig) -> (u8, String) {
    if let Some(license) = &metadata.license {
        for blocked in &config.blocklist {
            if license.contains(blocked) {
                return (0, format!("Blocked license: {}", license));
            }
        }
        return (100, format!("Allowed: {}", license));
    }
    (50, "Unknown license".to_string())
}
