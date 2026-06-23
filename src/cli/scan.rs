use clap::Parser;
use std::fs;
use crate::config;
use crate::http_client::HttpClient;
use crate::ecosystems::{self, Ecosystem};
use crate::scoring;
use serde_json::Value;

#[derive(Parser, Debug)]
pub struct ScanArgs {
    /// Fail if any package has a score below this threshold
    #[arg(long)]
    pub fail_under: Option<u8>,

    /// Check only newly added dependencies
    #[arg(long)]
    pub diff_only: bool,

    /// Output format (terminal, json, markdown)
    #[arg(short, long, default_value = "terminal")]
    pub format: String,
}

pub async fn run(args: ScanArgs) -> anyhow::Result<()> {
    let cfg = config::load_config().unwrap_or_default();
    let client = HttpClient::new(cfg.general.cache_ttl_hours)?;
    let mut failures = 0;
    let mut total_scanned = 0;

    let fail_threshold = args.fail_under.unwrap_or(cfg.thresholds.fail_under);

    // Scan package.json
    if let Ok(contents) = fs::read_to_string("package.json") {
        if let Ok(json) = serde_json::from_str::<Value>(&contents) {
            let mut deps = Vec::new();
            if let Some(dependencies) = json["dependencies"].as_object() {
                for (k, _) in dependencies {
                    deps.push(k.clone());
                }
            }
            if !deps.is_empty() {
                println!("Scanning package.json ({} dependencies)...", deps.len());
                for dep in deps {
                    total_scanned += 1;
                    if let Ok(metadata) = ecosystems::npm::fetch_metadata(&client, &dep).await {
                        let score = calculate_package_score(&client, &metadata, &cfg, Ecosystem::Npm).await;
                        print_scan_result(&dep, score, fail_threshold, &mut failures);
                    }
                }
            }
        }
    }

    // Scan Cargo.toml
    if let Ok(contents) = fs::read_to_string("Cargo.toml") {
        if let Ok(toml_val) = toml::from_str::<toml::Value>(&contents) {
            let mut deps = Vec::new();
            if let Some(dependencies) = toml_val.get("dependencies").and_then(|v| v.as_table()) {
                for (k, _) in dependencies {
                    deps.push(k.clone());
                }
            }
            if !deps.is_empty() {
                println!("Scanning Cargo.toml ({} dependencies)...", deps.len());
                for dep in deps {
                    total_scanned += 1;
                    if let Ok(metadata) = ecosystems::crates::fetch_metadata(&client, &dep).await {
                        let score = calculate_package_score(&client, &metadata, &cfg, Ecosystem::Crates).await;
                        print_scan_result(&dep, score, fail_threshold, &mut failures);
                    }
                }
            }
        }
    }

    if total_scanned == 0 {
        println!("No dependencies found to scan.");
        return Ok(());
    }

    println!("\nProject Risk Summary: {} failed, {} scanned total.", failures, total_scanned);
    
    if failures > 0 {
        std::process::exit(1);
    }

    Ok(())
}

async fn calculate_package_score(client: &HttpClient, metadata: &ecosystems::PackageMetadata, cfg: &config::Config, ecosystem: Ecosystem) -> u8 {
    let vulns = ecosystems::osv::check_vulnerabilities(client, &metadata.name, ecosystem.as_str(), &metadata.version).await.unwrap_or(0);
    
    let m_score = scoring::maintenance::calculate(metadata);
    let b_score = scoring::bus_factor::calculate(metadata);
    let c_score = scoring::community::calculate(metadata);
    let (l_score, _) = scoring::license::calculate(metadata, &cfg.license);
    let d_score = scoring::dependency_tree::calculate(vulns, metadata.dependencies.len());
    let a_score = 80;

    scoring::calculate_total_score(m_score, b_score, c_score, a_score, l_score, d_score)
}

fn print_scan_result(dep: &str, score: u8, threshold: u8, failures: &mut usize) {
    let status = if score < threshold {
        *failures += 1;
        "✗ FAIL"
    } else if score < 80 {
        "⚠ WARN"
    } else {
        "✓ PASS"
    };
    
    println!("  {:<25} {:>5}   {}", dep, score, status);
}
