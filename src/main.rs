use dotenv::dotenv;
use ethers::providers::{Provider, Ws};
use std::error::Error;
use trade_data_collector::{
    cli::parse_cli_args,
    config::get_ws_rpc_url,
    event_collector::{collect_order_events, write_to_csv},
    utils::{get_contract_creation_block, get_latest_block_number},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from `.env` file
    dotenv().ok();

    // Parse command-line arguments to determine network and contract details
    let args = parse_cli_args();

    // Retrieve WebSocket RPC URL based on the specified network
    let ws_rpc_url = get_ws_rpc_url(&args.network)?;

    // Initialize an Ethereum provider using WebSocket connection
    let provider = Provider::<Ws>::connect(&ws_rpc_url).await?;

    // Fetch the contract creation block using Etherscan API
    let api_key =
        std::env::var("ETHERSCAN_API_KEY").expect("ETHERSCAN_API_KEY environment variable not set");

    let base_url = std::env::var("ETHERSCAN_BASE_URL")
        .expect("ETHERSCAN_BASE_URL environment variable not set");

    let creation_block = get_contract_creation_block(&base_url, &api_key, &args.contract_address)?;

    // Get the latest block number from the Ethereum network
    let end_block = get_latest_block_number(&provider).await?;

    // Display contract creation and latest block information
    println!("Contract created at block: {}", creation_block);
    println!("Latest block: {}", end_block);

    // Collect order events within the block range
    let orders = collect_order_events(
        &ws_rpc_url,            // WebSocket RPC URL
        &args.contract_address, // Target contract address
        creation_block,         // Start block (contract deployment block)
        end_block,              // End block (latest block)
        1_000_000,              // Number of blocks to fetch per batch
        &args.event_type,       // Filter for specific event types (optional)
    )
    .await?;

    // Write collected events to a CSV file
    write_to_csv("order_events.csv", &orders)?;
    println!("✅ Data exported successfully!");

    Ok(())
}
