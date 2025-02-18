use anyhow::Result;
use clap::Parser;
use v040_memory::{app_builder, configuration::Configuration};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Configuration::parse();
    app_builder::run_app(configuration).await
}