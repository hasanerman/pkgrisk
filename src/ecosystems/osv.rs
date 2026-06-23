use crate::http_client::HttpClient;
use serde_json::Value;

pub async fn check_vulnerabilities(_client: &HttpClient, package: &str, ecosystem: &str, version: &str) -> anyhow::Result<usize> {
    // OSV.dev uses specific ecosystem names
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

    // We can't cache POST requests easily with our current HttpClient design,
    // so we'll just use a raw reqwest call or extend the client.
    // For now, let's use a standard reqwest client.
    let reqwest_client = reqwest::Client::new();
    let res = reqwest_client.post(url).json(&body).send().await?;
    let data: Value = res.json().await?;

    if let Some(vulns) = data["vulns"].as_array() {
        Ok(vulns.len())
    } else {
        Ok(0)
    }
}
