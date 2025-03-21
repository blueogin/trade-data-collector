use std::env;

/// Retrieves the WebSocket RPC URL for a given blockchain network.
///
/// This function checks the environment variables for the corresponding WebSocket
/// RPC URL of the specified network. If the network is unsupported or the environment
/// variable is not set, an error is returned.
///
/// # Arguments
///
/// * `network` - A string slice representing the blockchain network (e.g., "Mainnet", "Arbitrum").
///
/// # Returns
///
/// * `Ok(String)` - The WebSocket RPC URL if the environment variable is set.
/// * `Err(String)` - An error message if the network is unsupported or the environment variable is missing.
///
/// # Supported Networks
///
/// | Network   | Environment Variable       |
/// |-----------|---------------------------|
/// | Base      | BASE_WS_RPC_URL            |
/// | Mainnet   | MAINNET_WS_RPC_URL         |
/// | Flare     | FLARE_WS_RPC_URL           |
/// | Arbitrum  | ARBITRUM_WS_RPC_URL        |
/// | Optimism  | OPTIMISM_WS_RPC_URL        |
/// | Linear    | LINEAR_WS_RPC_URL          |
///
pub fn get_ws_rpc_url(network: &str) -> Result<String, String> {
  // Determine the corresponding environment variable for the given network
  let env_var = match network {
    "Base" => "BASE_WS_RPC_URL",
    "Mainnet" => "MAINNET_WS_RPC_URL",
    "Flare" => "FLARE_WS_RPC_URL",
    "Arbitrum" => "ARBITRUM_WS_RPC_URL",
    "Optimism" => "OPTIMISM_WS_RPC_URL",
    "Linear" => "LINEAR_WS_RPC_URL",
    _ => return Err(format!("Unsupported network: {}", network)),
  };

  // Retrieve the WebSocket RPC URL from environment variables
  env::var(env_var).map_err(|_| format!("{} not set", env_var))
}
