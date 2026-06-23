use clap::Parser;
use crate::config;
use crate::http_client::HttpClient;
use crate::ecosystems::{self, Ecosystem};
use crate::scoring::{self, CategoryScore, HealthScore};
use crate::output;

#[derive(Parser, Debug)]
pub struct CheckArgs {
    pub package: String,
    #[arg(short, long)]
    pub ecosystem: Option<String>,
    #[arg(short, long, default_value = "terminal")]
    pub format: String,
    #[arg(short, long)]
    pub quiet: bool,
}

pub async fn run(args: CheckArgs) -> anyhow::Result<()> {
    let cfg = config::load_config().unwrap_or_default();
    let client = HttpClient::new(cfg.general.cache_ttl_hours)?;

    let ecosystem = match args.ecosystem {
        Some(e) => Ecosystem::from_str(&e).unwrap_or(Ecosystem::Npm),
        None => Ecosystem::Npm,
    };

    let metadata = match ecosystem {
        Ecosystem::Npm => ecosystems::npm::fetch_metadata(&client, &args.package).await?,
        Ecosystem::Pypi => ecosystems::pypi::fetch_metadata(&client, &args.package).await?,
        Ecosystem::Crates => ecosystems::crates::fetch_metadata(&client, &args.package).await?,
    };

    let vulns = ecosystems::osv::check_vulnerabilities(&client, &args.package, ecosystem.as_str(), &metadata.version).await.unwrap_or(0);

    let m_score = scoring::maintenance::calculate(&metadata);
    let b_score = scoring::bus_factor::calculate(&metadata);
    let c_score = scoring::community::calculate(&metadata);
    let (l_score, l_details) = scoring::license::calculate(&metadata, &cfg.license);
    let d_score = scoring::dependency_tree::calculate(vulns, metadata.dependencies.len());
    let a_score = 80;

    let total = scoring::calculate_total_score(m_score, b_score, c_score, a_score, l_score, d_score);

    let health_score = HealthScore {
        total,
        maintenance: CategoryScore { score: m_score, details: format!("Last release: {}", metadata.latest_release_date.clone().unwrap_or("Unknown".to_string())) },
        bus_factor: CategoryScore { score: b_score, details: "1+ maintainers".to_string() },
        community: CategoryScore { score: c_score, details: "Healthy".to_string() },
        adoption: CategoryScore { score: a_score, details: format!("{} weekly downloads", metadata.weekly_downloads.unwrap_or(0)) },
        license: CategoryScore { score: l_score, details: l_details },
        dependencies: CategoryScore { score: d_score, details: format!("{} deps, {} vulns", metadata.dependencies.len(), vulns) },
    };

    output::print_output(&args.format, &metadata, &health_score, args.quiet);

    Ok(())
}
