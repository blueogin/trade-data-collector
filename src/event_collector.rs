use std::cmp::min;
use std::error::Error;

use csv::Writer;
use ethers::abi::Abi;
use ethers::providers::{Middleware, Provider, Ws};
use ethers::types::{BlockNumber, Filter, Log, H160, H256, U64};
use serde::Serialize;
use serde_json::Value;
use tokio::time::{sleep, Duration};

/// Represents a blockchain order event.
#[derive(Debug, Serialize)]
pub struct OrderEvent {
    pub tx_origin: H160,
    pub event_type: String,
    pub txn_hash: H256,
    pub timestamp: u64,
}

/// Loads the ABI from a JSON file and returns an `Abi` object.
fn load_abi(file_path: &str) -> Result<Abi, Box<dyn Error>> {
    let abi_json: Value = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;
    Ok(Abi::load(abi_json.to_string().as_bytes())?)
}

/// Retrieves event signatures based on the event type filter.
fn get_event_signatures(abi: &Abi, event_type: &str) -> Result<Vec<H256>, Box<dyn Error>> {
    let take_order_event = abi.event("TakeOrderV2")?.signature();
    let clear_event = abi.event("ClearV2")?.signature();

    let signatures = match event_type {
        "TakeOrderV2" => vec![take_order_event],
        "ClearV2" => vec![clear_event],
        "" => vec![take_order_event, clear_event], // Default: Both events
        &_ => vec![take_order_event, clear_event], // Default: Both events
    };

    Ok(signatures)
}

/// Fetches order events within a specified block range.
pub async fn collect_order_events(
    ws_rpc_url: &str,
    contract_address: &str,
    from_block: u64,
    to_block: u64,
    chunk_size: u64,
    event_type: &str,
) -> Result<Vec<OrderEvent>, Box<dyn Error>> {
    let provider = Provider::<Ws>::connect(ws_rpc_url).await?;
    let contract_addr: H160 = contract_address.parse()?;
    let mut events = Vec::new();

    let abi = load_abi("./IOrderBookV4.json")?;
    let event_signatures = get_event_signatures(&abi, event_type)?;

    let take_order_event = abi.event("TakeOrderV2")?;
    let clear_event = abi.event("ClearV2")?;

    let mut start_block = from_block;

    while start_block <= to_block {
        let end_block = min(start_block + chunk_size - 1, to_block);
        let filter = Filter::new()
            .address(contract_addr)
            .topic0(event_signatures.clone())
            .from_block(BlockNumber::Number(U64::from(start_block)))
            .to_block(BlockNumber::Number(U64::from(end_block)));

        match provider.get_logs(&filter).await {
            Ok(logs) => {
                process_logs(&provider, logs, take_order_event, clear_event, &mut events).await;
            }
            Err(e) => {
                eprintln!(
                    "Error fetching logs for blocks {} to {}: {:?}",
                    start_block, end_block, e
                );
            }
        }

        start_block = end_block + 1;
        // sleep(Duration::from_millis(100)).await; // Avoid rate limits
    }

    Ok(events)
}

/// Processes logs and extracts order event data.
async fn process_logs(
    provider: &Provider<Ws>,
    logs: Vec<Log>,
    take_order_event: &ethers::abi::Event,
    _clear_event: &ethers::abi::Event,
    events: &mut Vec<OrderEvent>,
) {
    for log in logs {
        let detected_event = if log.topics[0] == take_order_event.signature() {
            "TakeOrderV2"
        } else {
            "ClearV2"
        };

        if let Some(block_number) = log.block_number {
            if let Ok(Some(block)) = provider.get_block(block_number).await {
                println!(
                    "Event Type: {}  Block Number: {}",
                    detected_event, block_number
                );
                events.push(OrderEvent {
                    tx_origin: log.address,
                    event_type: detected_event.to_string(),
                    txn_hash: log.transaction_hash.unwrap_or_default(),
                    timestamp: block.timestamp.as_u64(),
                });
            }
        }
    }
}

/// Writes order events to a CSV file.
pub fn write_to_csv(filename: &str, events: &[OrderEvent]) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(filename)?;
    writer.write_record(["tx.origin", "event type", "txn hash", "timestamp"])?;

    for event in events {
        writer.write_record(&[
            format!("{:?}", event.tx_origin),
            event.event_type.clone(),
            format!("{:?}", event.txn_hash),
            event.timestamp.to_string(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}
