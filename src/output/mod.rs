pub mod json;
pub mod markdown;
pub mod terminal;

use crate::scoring::HealthScore;
use crate::ecosystems::PackageMetadata;

pub fn print_output(format: &str, metadata: &PackageMetadata, score: &HealthScore, quiet: bool) {
    if quiet {
        let verdict = if score.total >= 80 { "Healthy" } else if score.total >= 50 { "Caution" } else { "Risky" };
        println!("{} {} - {}/100 ({})", metadata.name, metadata.version, score.total, verdict);
        return;
    }

    match format {
        "json" => json::print(metadata, score),
        "markdown" => markdown::print(metadata, score),
        _ => terminal::print(metadata, score),
    }
}
