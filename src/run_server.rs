use anyhow::Result;
use clap::Parser;
use nym_proxy::NymProxyServer;

#[derive(Parser, Debug)]
struct Args {
    /// Upstream address, ie lightwalletd address
    #[clap(short, long)]
    upstream_address: String,

    /// Config directory
    #[clap(short, long, default_value = "/tmp/mixnet-client")]
    config_dir: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    nym_bin_common::logging::setup_logging();

    let args = Args::parse();

    let mut proxy_server = NymProxyServer::new(&args.upstream_address, &args.config_dir).await?;

    proxy_server.run_with_shutdown().await
}
