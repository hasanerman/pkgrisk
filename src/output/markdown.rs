use crate::ecosystems::PackageMetadata;
use crate::scoring::HealthScore;

pub fn print(metadata: &PackageMetadata, score: &HealthScore) {
    println!("## {}@{}", metadata.name, metadata.version);
    println!("**Health Score:** {}/100", score.total);
    println!();
    println!("| Category | Score | Details |");
    println!("|---|---|---|");
    println!("| Maintenance | {} | {} |", score.maintenance.score, score.maintenance.details);
    println!("| Bus Factor | {} | {} |", score.bus_factor.score, score.bus_factor.details);
    println!("| Community | {} | {} |", score.community.score, score.community.details);
    println!("| Adoption | {} | {} |", score.adoption.score, score.adoption.details);
    println!("| License | {} | {} |", score.license.score, score.license.details);
    println!("| Dependencies | {} | {} |", score.dependencies.score, score.dependencies.details);
}
