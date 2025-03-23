use dotenv::dotenv;
use std::error::Error;
use trade_data_collector::{
    cli::parse_cli_args,
    constants,
    event_collector::collect_order_events,
    utils::get_ws_rpc_url,
    utils::{get_contract_creation_block, get_latest_block_number},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from `.env` file
    dotenv().ok();
    println!("{}",std::env::var("ETHERSCAN_API_KEY"));
    println!("{}",std::env::var("BASESCAN_API_KEY"));

    println!("{}",std::env::var("INFURA_API_KEY"));


    // Parse command-line arguments to determine network and contract details
    let args = parse_cli_args();

    // Retrieve WebSocket RPC URL based on the specified network
    let ws_rpc_url = get_ws_rpc_url(&args.network)?;

    // Fetch the contract creation block using Etherscan API
    let api_key =
        std::env::var("ETHERSCAN_API_KEY").expect("ETHERSCAN_API_KEY environment variable not set");

    let creation_block = get_contract_creation_block(
        constants::ETHERSCAN_BASIC_URL,
        &api_key,
        &args.contract_address,
    )?;

    // Get the latest block number from the Ethereum network
    let end_block = get_latest_block_number(&ws_rpc_url).await?;

    // Display contract creation and latest block information
    println!("Contract created at block: {}", creation_block);
    println!("Latest block: {}", end_block);

    // Collect order events within the block range
    collect_order_events(
        &ws_rpc_url,                 // WebSocket RPC URL
        &args.contract_address,      // Target contract address
        creation_block,              // Start block (contract deployment block)
        end_block,                   // End block (latest block)
        1_000_000,                   // Number of blocks to fetch per batch
        &args.event_type,            // Filter for specific event types (optional)
        constants::OUTPUT_FILE_PATH, // Output csv file path
    )
    .await?;

    Ok(())
}
