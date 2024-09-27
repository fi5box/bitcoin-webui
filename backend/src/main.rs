mod bitcoin_info;
mod config;

use std::time::Duration;

use axum::{extract::State, routing::get, Router};
use clap::Parser;

use tower_http::{
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use bitcoin_info::BitcoinInfo;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::info;

/// A subcommand for run
#[derive(Parser)]
struct RunOpts {
    /// Chain config path
    #[clap(short = 'c', long = "config", default_value = "config.toml")]
    config_file: String,
}

#[derive(Parser)]
enum SubCommand {
    /// run this service
    #[clap(name = "run")]
    Run(RunOpts),
}

pub fn clap_about() -> String {
    let name = env!("CARGO_PKG_NAME").to_string();
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    name + " " + version + "\n" + authors
}

#[derive(Parser)]
#[clap(version, about = clap_about())]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() {
    ::std::env::set_var("RUST_BACKTRACE", "full");

    env_logger::init();

    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Run(opts) => {
            run(opts);
        }
    }
}

#[tokio::main]
async fn run(opts: RunOpts) {
    let config = config::load_config(opts.config_file);

    let shared_state = Arc::new(RwLock::new(BitcoinInfo::default()));

    let router = Router::new()
        // static
        .nest_service(
            "/",
            ServeDir::new(&config.static_dir)
                .not_found_service(ServeFile::new(format!("{}/index.html", &config.static_dir))),
        )
        .route("/bitcoin", get(bitcoin_info))
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(config.request_timeout)),
        ))
        // state
        .with_state(Arc::clone(&shared_state));

    let listener = tokio::net::TcpListener::bind(format!("[::]:{}", config.port))
        .await
        .unwrap();

    info!("start server ...");

    axum::serve(listener, router).await.unwrap();
}

async fn bitcoin_info(State(state): State<Arc<RwLock<BitcoinInfo>>>) -> String {
    let bitcoin_info = state.read().await;

    serde_json::to_string(&(*bitcoin_info)).unwrap()
}
