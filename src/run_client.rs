use anyhow::Result;
use clap::Parser;
use nym_proxy::NymProxyClient;

#[derive(Parser, Debug)]
struct Args {
    /// Send timeout in seconds
    #[clap(long, default_value_t = 10)]
    close_timeout: u64,

    /// Mixnet server address
    #[clap(short, long)]
    server_address: String,

    /// Listen address
    #[clap(long, default_value = "127.0.0.1")]
    listen_address: String,

    /// Listen port
    #[clap(long, default_value = "8080")]
    listen_port: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    nym_bin_common::logging::setup_logging();
    let args = Args::parse();

    let proxy_client = NymProxyClient::new(
        &args.server_address,
        &args.listen_address,
        &args.listen_port,
        args.close_timeout,
    )
    .await?;

    proxy_client.run().await
}
