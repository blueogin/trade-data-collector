use std::cmp::min;
use std::error::Error;

use ethers::abi::Abi;
use ethers::providers::{Middleware, Provider, Ws};
use ethers::types::{BlockNumber, Filter, Log, H160, H256, U64};
use log::{error, info};
use serde_json::Value;
use tokio::time::{sleep, Duration};

use crate::constants;
use crate::csv_manager::{initialize_csv, write_to_csv};
use crate::utils::OrderEvent;

/// Loads the ABI from a JSON file and returns an `Abi` object.
fn load_abi(file_path: &str) -> Result<Abi, Box<dyn Error>> {
    let abi_json: Value = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;
    Ok(Abi::load(abi_json.to_string().as_bytes())?)
}

/// Retrieves event signatures based on the event type filter.
fn get_event_signatures(abi: &Abi, event_type: &str) -> Result<Vec<H256>, Box<dyn Error>> {
    let take_order_event = abi.event(constants::TAKEORDER_EVENT_NAME)?.signature();
    let clear_event = abi.event(constants::CLEAR_EVENT_NAME)?.signature();

    let signatures = match event_type {
        constants::TAKEORDER_EVENT_NAME => vec![take_order_event],
        constants::CLEAR_EVENT_NAME => vec![clear_event],
        constants::DEFAULT => vec![take_order_event, clear_event], // Default: Both events
        &_ => vec![take_order_event, clear_event],
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
    filename: &str, // Add filename parameter
) -> Result<(), Box<dyn Error>> {
    let provider = Provider::<Ws>::connect(ws_rpc_url).await?;
    let contract_addr: H160 = contract_address.parse()?;
    let abi = load_abi(constants::ABI_FILE_PATH)?;
    let event_signatures = get_event_signatures(&abi, event_type)?;

    let take_order_event = abi.event("TakeOrderV2")?;
    let clear_event = abi.event("ClearV2")?;

    let mut start_block = from_block;

    // Initialize CSV file once before appending
    initialize_csv(filename)?;

    info!(
        "Collecting Event data from {} to {} with chunk size of {} for {} contract",
        from_block, to_block, chunk_size, contract_address,
    );
    while start_block <= to_block {
        let end_block = min(start_block + chunk_size - 1, to_block);

        info!(
            "    Collecting Event data from {} to {}",
            start_block, end_block,
        );
        let filter = Filter::new()
            .address(contract_addr)
            .topic0(event_signatures.clone())
            .from_block(BlockNumber::Number(U64::from(start_block)))
            .to_block(BlockNumber::Number(U64::from(end_block)));

        let mut events = Vec::new(); // Clear events per chunk

        match provider.get_logs(&filter).await {
            Ok(logs) => {
                process_logs(&provider, logs, take_order_event, clear_event, &mut events).await;

                // Append chunk data to CSV
                if !events.is_empty() {
                    write_to_csv(filename, &events)?;
                }
            }
            Err(e) => {
                error!(
                    "Error fetching logs for blocks {} to {}: {:?}",
                    start_block, end_block, e
                );
            }
        }

        info!(
            "    Ending Event data from {} to {}",
            start_block, end_block,
        );
        start_block = end_block + 1;
        sleep(Duration::from_millis(100)).await; // Avoid rate limits
    }

    info!(
        "Ending Event data from {} to {} with chunk size of {} for {} contract",
        from_block, to_block, chunk_size, contract_address,
    );
    info!("✅ Data exported successfully!");
    Ok(())
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
                if let Some(txn_hash) = log.transaction_hash {
                    if let Ok(Some(txn)) = provider.get_transaction(txn_hash).await {
                        let event = OrderEvent {
                            tx_origin: txn.from,
                            event_type: detected_event.to_string(),
                            txn_hash,
                            timestamp: block.timestamp.as_u64(),
                        };

                        info!(
                            "        Tx Hash: {}  Event Type: {}",
                            event.txn_hash, event.event_type
                        );

                        events.push(event);
                    }
                }
            }
        }
    }
}
