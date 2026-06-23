use super::PackageMetadata;
use crate::http_client::HttpClient;
use serde_json::Value;

pub async fn fetch_metadata(client: &HttpClient, package: &str) -> anyhow::Result<PackageMetadata> {
    let url = format!("https://pypi.org/pypi/{}/json", package);
    let data: Value = client.get_json(&url).await?;

    let info = &data["info"];
    let version = info["version"].as_str().unwrap_or("unknown").to_string();
    
    let mut dependencies = Vec::new();
    if let Some(reqs) = info["requires_dist"].as_array() {
        for req in reqs {
            if let Some(req_str) = req.as_str() {
                // simple split for basic parsing
                let name = req_str.split_whitespace().next().unwrap_or("").to_string();
                if !name.is_empty() {
                    dependencies.push(name);
                }
            }
        }
    }

    let repo_url = info["project_urls"].as_object().and_then(|urls| {
        urls.get("Source").or(urls.get("Homepage")).and_then(|v| v.as_str()).map(|s| s.to_string())
    });

    let releases = &data["releases"][&version];
    let latest_release_date = releases.as_array()
        .and_then(|arr| arr.first())
        .and_then(|obj| obj["upload_time"].as_str())
        .map(|s| s.to_string());

    Ok(PackageMetadata {
        name: package.to_string(),
        version,
        description: info["summary"].as_str().map(|s| s.to_string()),
        license: info["license"].as_str().map(|s| s.to_string()),
        repository_url: repo_url,
        latest_release_date,
        dependencies,
        weekly_downloads: None, // PyPI doesn't expose downloads via this API easily
    })
}
