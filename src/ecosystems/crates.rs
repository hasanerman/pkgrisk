use super::PackageMetadata;
use crate::http_client::HttpClient;
use serde_json::Value;

pub async fn fetch_metadata(client: &HttpClient, package: &str) -> anyhow::Result<PackageMetadata> {
    let url = format!("https://crates.io/api/v1/crates/{}", package);
    let data: Value = client.get_json(&url).await?;

    let crate_info = &data["crate"];
    let version = crate_info["max_version"].as_str().unwrap_or("unknown").to_string();
    
    // Dependencies require another API call
    let deps_url = format!("https://crates.io/api/v1/crates/{}/{}/dependencies", package, version);
    let deps_data: Value = client.get_json(&deps_url).await.unwrap_or(serde_json::json!({"dependencies": []}));
    
    let mut dependencies = Vec::new();
    if let Some(deps) = deps_data["dependencies"].as_array() {
        for dep in deps {
            if let Some(kind) = dep["kind"].as_str() {
                if kind == "normal" {
                    if let Some(name) = dep["crate_id"].as_str() {
                        dependencies.push(name.to_string());
                    }
                }
            }
        }
    }

    Ok(PackageMetadata {
        name: package.to_string(),
        version,
        description: crate_info["description"].as_str().map(|s| s.to_string()),
        license: crate_info["exact_match_license"].as_str().or(crate_info["license"].as_str()).map(|s| s.to_string()),
        repository_url: crate_info["repository"].as_str().map(|s| s.to_string()),
        latest_release_date: crate_info["updated_at"].as_str().map(|s| s.to_string()),
        dependencies,
        weekly_downloads: crate_info["recent_downloads"].as_u64(),
    })
}
