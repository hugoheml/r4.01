use clap::Parser;
use v050_use_cases::{app_builder::run_app, configuration::Configuration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = Configuration::parse();
    run_app(configuration).await
}