use anchor_client::{solana_sdk::commitment_config::CommitmentConfig, Cluster};
use futures_util::StreamExt;
use snafu::{ResultExt, Snafu};
use solana_client::{
    nonblocking::pubsub_client::{PubsubClient, PubsubClientError},
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};
use tracing::info;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to connect to websocket at {}: {}", url, source))]
    WebsocketConnection { url: String, source: PubsubClientError },

    #[snafu(display("Failed to subscribe to logs: {}", source))]
    LogSubscription { source: PubsubClientError },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Subscribe to Solana program logs for the pump_amm program
///
/// # Errors
/// Returns an error if:
/// - Cannot connect to the websocket endpoint
/// - Cannot subscribe to program logs
/// - Stream ends unexpectedly
pub async fn subscribe() -> Result<()> {
    let program_id = pump_amm::ID.to_string();
    let ws_url = Cluster::Mainnet.ws_url();

    let ws_client =
        PubsubClient::new(ws_url).await.context(WebsocketConnectionSnafu { url: ws_url })?;

    let (mut stream, _) = ws_client
        .logs_subscribe(
            RpcTransactionLogsFilter::Mentions(vec![program_id.clone()]),
            RpcTransactionLogsConfig { commitment: Some(CommitmentConfig::confirmed()) },
        )
        .await
        .context(LogSubscriptionSnafu)?;

    while let Some(log) = stream.next().await {
        let events = pump_amm::parse_logs_response(&log, &program_id);
        for event in events {
            info!("Received {event:?}");
        }
    }

    Ok(())
}
