use ethers::types::{BlockNumber, BlockId};
use std::error::Error;
use serde_json::Value;
use ethers::providers::{Provider, Ws, Middleware};
use ureq;

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
pub fn get_contract_creation_block(base_url: &str, api_key: &str, contract_address: &str) -> Result<u64, Box<dyn Error>> {

  let url = format!(
    "{}/api?module=contract&action=getcontractcreation&contractaddresses={}&apikey={}",
    base_url, contract_address, api_key
  );
  
  // Send the request to Etherscan API and parse the JSON response
  let res: Value = ureq::get(&url).call()?.into_json()?;

  // Check if the API response status is successful
  if res["status"] == "1" {
    // Extract and parse the block number
    if let Some(block_number_str) = res["result"][0]["blockNumber"].as_str() {
        block_number_str.parse::<u64>().map_err(|_| "Failed to parse block number".into())
    } else {
        Err("Block number not found in contract creation details.".into())
    }
  } else {
    Err(format!("Failed to retrieve contract creation transaction: {}", res["message"]).into())
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
pub async fn get_latest_block_number(provider: &Provider<Ws>) -> Result<u64, Box<dyn Error>> {
  match provider.get_block(BlockId::Number(BlockNumber::Latest)).await? {
    Some(block) => Ok(block.number.unwrap_or_default().as_u64()), // Extracts and returns the block number
    None => Err("Failed to fetch the latest block".into()),
  }
}
