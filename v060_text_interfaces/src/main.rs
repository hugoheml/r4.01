use clap::Parser;
use v060_text_interfaces::{app_builder::run_app, configuration::Configuration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = Configuration::parse();
    run_app(configuration).await
}