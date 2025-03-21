use clap::{Arg, Command};

/// Structure to hold command-line arguments for the Trade Data Collector.
pub struct CliArgs {
  /// The blockchain network to connect to (e.g., Mainnet, Testnet).
  pub network: String,
  /// The smart contract address from which to track events.
  pub contract_address: String,
  /// The specific event type to filter (e.g., TakeOrderV2, ClearV2).
  pub event_type: String,
}

/// Parses command-line arguments and returns a `CliArgs` struct.
///
/// This function utilizes the `clap` crate to handle CLI arguments.
/// It defines and retrieves values for the following options:
/// - `--network` (`-n`): Specifies the blockchain network (default: Mainnet).
/// - `--contract` (`-c`): Specifies the smart contract address (required).
/// - `--event` (`-e`): Specifies the event type to filter (optional).
///
/// # Returns
/// A `CliArgs` struct containing the parsed values from the command line.
pub fn parse_cli_args() -> CliArgs {
  let matches = Command::new("Trade Data Collector")
    .version("1.0")
    .author("Your Name")
    .about("Collects and exports trade order events from a DEX")
    .arg(
      Arg::new("network")
        .short('n')
        .long("network")
        .num_args(1)
        .value_name("NETWORK_NAME")
        .default_value("Mainnet")
        .help("Specifies the blockchain network to use (default: Mainnet)"),
    )
    .arg(
      Arg::new("contract")
        .short('c')
        .long("contract")
        .num_args(1)
        .value_name("CONTRACT_ADDRESS")
        .required(true)
        .help("The smart contract address to track events from"),
    )
    .arg(
      Arg::new("event")
        .short('e')
        .long("event")
        .value_name("EVENT_TYPE")
        .default_value("")
        .help("Filters by a specific event type (e.g., TakeOrderV2, ClearV2)"),
    )
    .get_matches();

  // Extract and return CLI arguments
  CliArgs {
    network: matches.get_one::<String>("network").unwrap().clone(),
    contract_address: matches.get_one::<String>("contract").unwrap().clone(),
    event_type: matches.get_one::<String>("event").unwrap().clone(),
  }
}
