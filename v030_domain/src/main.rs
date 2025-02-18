use anyhow::Result;
use clap::Parser;
use v030_domain::{app_builder, configuration::Configuration};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Configuration::parse();
    app_builder::run_app(configuration).await
}