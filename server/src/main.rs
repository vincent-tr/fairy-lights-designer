mod config;
mod web_api;
mod web_error;
mod web_static;

use anyhow::Result;
use axum::Router;
use clap::Parser;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;

use config::Config;
use web_error::WebError;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();

    let app = Router::new()
        .merge(web_api::build(&config.mongo_url).await?)
        .merge(web_static::build());

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, config.web_port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
