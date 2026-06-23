use clap::Parser;

#[derive(Parser, Debug)]
pub struct CompareArgs {
    /// Packages to compare
    #[arg(required = true)]
    pub packages: Vec<String>,
}

pub async fn run(args: CompareArgs) -> anyhow::Result<()> {
    println!("Compare command running for: {:?}", args.packages);
    Ok(())
}
