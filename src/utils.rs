use std::error::Error;
use std::fs;

use ethers::providers::{Middleware, Provider, Ws};
use ethers::types::{BlockId, BlockNumber};
use ethers::types::{H160, H256};

use serde::Serialize;
use serde_json::Value;
use ureq;

use crate::constants;
use ethers_contract::Abigen;

/// Represents a blockchain order event.
#[derive(Debug, Serialize)]
pub struct OrderEvent {
    pub tx_origin: H160,
    pub event_type: String,
    pub txn_hash: H256,
    pub timestamp: u64,
}
/// Retrieves the WebSocket RPC URL for a given blockchain network.
///
/// This function checks the constant variables for the corresponding WebSocket
/// RPC URL of the specified network. If the network is unsupported or the constant
/// variable is not set, an error is returned.
///
/// # Arguments
///
/// * `network` - A string slice representing the blockchain network (e.g., "Mainnet", "Arbitrum").
///
/// # Returns
///
/// * `Ok(String)` - The WebSocket RPC URL if the constant variable is set.
/// * `Err(String)` - An error message if the network is unsupported or the constant variable is missing.
///
/// # Supported Networks
///
/// | Network   | Constants       |
/// |-----------|---------------------------|
/// | Base      | BASE_WS_RPC_URL            |
/// | Mainnet   | MAINNET_WS_RPC_URL         |
/// | Arbitrum  | ARBITRUM_WS_RPC_URL        |
/// | Optimism  | OPTIMISM_WS_RPC_URL        |
/// | Linear    | LINEAR_WS_RPC_URL          |
///
pub fn get_ws_rpc_url(network: &str) -> Result<String, String> {
    // Determine the corresponding constant variable for the given network
    let basic_url = match network {
        "Base" => constants::BASE_WS_RPC_BASE_URL,
        "Mainnet" => constants::MAINNET_WS_RPC_BASIC_URL,
        "Arbitrum" => constants::ARBITRUM_WS_RPC_BASE_URL,
        "Optimism" => constants::OPTIMISM_WS_RPC_BASE_URL,
        "Linear" => constants::LINEA_WS_RPC_BASE_URL,
        _ => return Err(format!("Unsupported network: {}", network)),
    };

    // Retrieve the WebSocket RPC URL from constant variable
    Ok(format!(
        "{}{}",
        basic_url,
        std::env::var("INFURA_API_KEY").expect("INFURA_API_KEY environment variable not set")
    ))
}

/// Retrieves the block number where a given smart contract was first deployed.
///
/// This function queries the Etherscan API to fetch the contract creation details.
///
/// # Arguments
///
/// * `api_key` - A string slice containing the Etherscan API key.
/// * `contract_address` - The address of the smart contract in hexadecimal format.
///
/// # Returns
///
/// * `Ok(u64)` - The block number where the contract was deployed.
/// * `Err(Box<dyn Error>)` - An error message if the API request fails or the block number is not found.
///
pub fn get_contract_creation_block(
    base_url: &str,
    api_key: &str,
    contract_address: &str,
) -> Result<u64, Box<dyn Error>> {
    let url = format!(
        "{}/api?module=contract&action=getcontractcreation&contractaddresses={}&apikey={}",
        base_url, contract_address, api_key
    );

    // Send the request to Etherscan API and parse the JSON response
    let res: String = ureq::get(&url).call()?.into_string()?;
    let res: Value = serde_json::from_str(&res)?;

    // Check if the API response status is successful
    if res["status"] == "1" {
        // Extract and parse the block number
        if let Some(block_number_str) = res["result"][0]["blockNumber"].as_str() {
            block_number_str
                .parse::<u64>()
                .map_err(|_| "Failed to parse block number".into())
        } else {
            Err("Block number not found in contract creation details.".into())
        }
    } else {
        Err(format!(
            "Failed to retrieve contract creation transaction: {}",
            res["message"]
        )
        .into())
    }
}

/// Fetches the latest block number from the Ethereum blockchain.
///
/// This function connects to an Ethereum node via WebSocket and retrieves the latest block number.
///
/// # Arguments
///
/// * `provider` - A reference to an `ethers::providers::Provider<Ws>` instance to interact with the blockchain.
///
/// # Returns
///
/// * `Ok(u64)` - The latest block number on the chain.
/// * `Err(Box<dyn Error>)` - An error message if the latest block cannot be fetched.
///
pub async fn get_latest_block_number(ws_rpc_url: &str) -> Result<u64, Box<dyn Error>> {
    let provider = Provider::<Ws>::connect(ws_rpc_url).await?;
    match provider
        .get_block(BlockId::Number(BlockNumber::Latest))
        .await?
    {
        Some(block) => Ok(block.number.unwrap_or_default().as_u64()), // Extracts and returns the block number
        None => Err("Failed to fetch the latest block".into()),
    }
}

/// Loads an ABI (Application Binary Interface) file and generates Rust contract bindings.
///
/// # Arguments
/// * `abi_path` - A string slice that holds the path to the ABI file.
///
/// # Returns
/// * `Result<String, Box<dyn Error>>` - A Result containing the generated contract bindings as a String or an error.
///
/// # Errors
/// * Returns an error if the file cannot be read or if the ABI parsing fails.
pub fn load_abi(abi_path: &str) -> Result<String, Box<dyn Error>> {
    // Read the ABI file content into a string
    let abi_content = fs::read_to_string(abi_path)?;

    // Create an Abigen instance and generate Rust bindings for the contract
    let bindings = Abigen::new("MyContract", abi_content)?.generate()?;

    // Return the generated bindings as a string
    Ok(bindings.to_string())
}
