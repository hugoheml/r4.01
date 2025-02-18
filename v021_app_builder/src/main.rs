use clap::Parser;
use v021_app_builder::{app_builder, configuration::Configuration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = Configuration::parse();
    app_builder::run_app(configuration).await
}