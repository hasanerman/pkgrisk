use super::PackageMetadata;
use crate::http_client::HttpClient;
use serde_json::Value;

pub async fn fetch_metadata(client: &HttpClient, package: &str) -> anyhow::Result<PackageMetadata> {
    let url = format!("https://registry.npmjs.org/{}", package);
    let data: Value = client.get_json(&url).await?;

    let latest_version = data["dist-tags"]["latest"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();

    let latest_data = &data["versions"][&latest_version];

    let mut dependencies = Vec::new();
    if let Some(deps) = latest_data["dependencies"].as_object() {
        for (k, _) in deps {
            dependencies.push(k.clone());
        }
    }

    let repo_url = data["repository"]["url"].as_str().map(|s| {
        s.replace("git+", "").replace("git://", "https://")
    });

    let time_obj = data["time"].as_object();
    let release_date = time_obj.and_then(|t| t.get(&latest_version)).and_then(|s| s.as_str()).map(|s| s.to_string());

    Ok(PackageMetadata {
        name: package.to_string(),
        version: latest_version,
        description: data["description"].as_str().map(|s| s.to_string()),
        license: data["license"].as_str().map(|s| s.to_string()),
        repository_url: repo_url,
        latest_release_date: release_date,
        dependencies,
        weekly_downloads: fetch_downloads(client, package).await.ok(),
    })
}

async fn fetch_downloads(client: &HttpClient, package: &str) -> anyhow::Result<u64> {
    let url = format!("https://api.npmjs.org/downloads/point/last-week/{}", package);
    let data: Value = client.get_json(&url).await?;
    Ok(data["downloads"].as_u64().unwrap_or(0))
}
