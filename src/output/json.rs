use crate::ecosystems::PackageMetadata;
use crate::scoring::HealthScore;
use serde_json::json;

pub fn print(metadata: &PackageMetadata, score: &HealthScore) {
    let output = json!({
        "name": metadata.name,
        "version": metadata.version,
        "score": score.total,
        "categories": score
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
