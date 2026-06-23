use crate::http_client::HttpClient;
use serde_json::Value;

pub async fn check_vulnerabilities(_client: &HttpClient, package: &str, ecosystem: &str, version: &str) -> anyhow::Result<usize> {
    let osv_ecosystem = match ecosystem {
        "npm" => "npm",
        "pypi" => "PyPI",
        "crates" => "crates.io",
        _ => return Ok(0),
    };

    let url = "https://api.osv.dev/v1/query";
    let body = serde_json::json!({
        "version": version,
        "package": {
            "name": package,
            "ecosystem": osv_ecosystem
        }
    });

    let reqwest_client = reqwest::Client::new();
    let res = reqwest_client.post(url).json(&body).send().await?;
    let data: Value = res.json().await?;

    if let Some(vulns) = data["vulns"].as_array() {
        Ok(vulns.len())
    } else {
        Ok(0)
    }
}
