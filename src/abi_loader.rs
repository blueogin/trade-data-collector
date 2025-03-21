use ethers_contract::Abigen;
use std::fs;
use std::error::Error;

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
///
/// # Example
/// ```rust
/// let abi_bindings = load_abi("./path/to/contract.abi").expect("Failed to load ABI");
/// ```
pub fn load_abi(abi_path: &str) -> Result<String, Box<dyn Error>> {
  // Read the ABI file content into a string
  let abi_content = fs::read_to_string(abi_path)?;
  
  // Create an Abigen instance and generate Rust bindings for the contract
  let bindings = Abigen::new("MyContract", abi_content)?.generate()?;
  
  // Return the generated bindings as a string
  Ok(bindings.to_string())
}
