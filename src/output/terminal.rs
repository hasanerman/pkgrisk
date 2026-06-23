use crate::ecosystems::PackageMetadata;
use crate::scoring::HealthScore;
use colored::*;

pub fn print(metadata: &PackageMetadata, score: &HealthScore) {
    println!("\n  {}@{} ({})", metadata.name.bold(), metadata.version, "npm/pypi/crates".dimmed());
    
    let verdict = if score.total >= 80 {
        "✓ HEALTHY".green()
    } else if score.total >= 50 {
        "⚠ CAUTION".yellow()
    } else {
        "✗ RISKY".red()
    };

    println!("\n  Health Score: {}/100  {}\n", score.total, verdict);

    let format_score = |s: u8| -> ColoredString {
        if s >= 80 { "✓".green() } else if s >= 50 { "⚠".yellow() } else { "✗".red() }
    };

    println!("  ├─ Maintenance     {}  {}", format_score(score.maintenance.score), score.maintenance.details);
    println!("  ├─ Bus Factor      {}  {}", format_score(score.bus_factor.score), score.bus_factor.details);
    println!("  ├─ Community       {}  {}", format_score(score.community.score), score.community.details);
    println!("  ├─ Adoption        {}  {}", format_score(score.adoption.score), score.adoption.details);
    println!("  ├─ License         {}  {}", format_score(score.license.score), score.license.details);
    println!("  └─ Dep Tree Risk   {}  {}\n", format_score(score.dependencies.score), score.dependencies.details);
}
