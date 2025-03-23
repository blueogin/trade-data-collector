use super::*;
use ethers::types::{H160, H256};
use event_collector::collect_order_events;
use hex::decode;
use mockito::Server;
use proptest::prelude::*;
use proptest::prop_oneof;
use proptest::test_runner::{Config, TestRunner};
use std::error::Error;
use std::io::Read;
use tempfile::NamedTempFile;

use csv_manager::{initialize_csv, verify_csv, write_to_csv};
use utils::{get_contract_creation_block, get_latest_block_number, OrderEvent};

#[test]
/// **Unit Test**: Verifies that the function `get_contract_creation_block` works
/// correctly when the API returns a successful response with a block number.
fn test_get_contract_creation_block_success() {
    let mut server = Server::new();
    let api_key = "test_api_key";
    let contract_address = "0x1234567890abcdef";

    // Mock API response with block number for contract creation
    let mock_resp = r#"
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "contractAddress": "0x1234567890abcdef",
        "blockNumber": "12345678"
      }
    ]
  }"#;

    let url = format!(
        "/api?module=contract&action=getcontractcreation&contractaddresses={}&apikey={}",
        contract_address, api_key
    );
    let mock_endpoint = server
        .mock("GET", &*url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_resp)
        .create();

    // Call the function with the mock server URL
    let result = get_contract_creation_block(&server.url(), api_key, contract_address);

    // Verify that the result is as expected
    assert_eq!(result.unwrap(), 12345678);

    // Ensure the mock endpoint was hit
    mock_endpoint.assert();
}

#[test]
/// **Unit Test**: Tests the failure scenario for `get_contract_creation_block` when the
/// API returns an error status.
fn test_get_contract_creation_block_failure() {
    let mut server = Server::new();
    let api_key = "test_api_key";
    let contract_address = "0x1234567890abcdef";

    // Mock API response with an error status
    let mock_resp = r#"
  {
    "status": "0",
    "message": "OK",
    "result": [
      {
        "contractAddress": "0x1234567890abcdef",
        "blockNumber": "12345678"
      }
    ]
  }"#;

    let url = format!(
        "/api?module=contract&action=getcontractcreation&contractaddresses={}&apikey={}",
        contract_address, api_key
    );
    let mock_endpoint = server
        .mock("GET", &*url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_resp)
        .create();

    // Call the function with the mock server URL
    let result = get_contract_creation_block(&server.url(), api_key, contract_address);

    // Assert that the result is an error
    assert!(result.is_err());

    // Ensure the mock endpoint was hit
    mock_endpoint.assert();
}

#[tokio::test]
/// **Integration Test**: Tests the `get_latest_block_number` function by connecting to
/// a WebSocket provider and checking if the block number is greater than 0.
async fn test_get_latest_block_number() -> Result<(), Box<dyn std::error::Error>> {
    let ws_rpc_url = format!(
        "{}{}",
        constants::MAINNET_WS_RPC_BASIC_URL,
        constants::TEST_INFURA_API_KEY
    );
    let latest_block = get_latest_block_number(&ws_rpc_url).await?;

    // Assert that the latest block number is greater than 0
    assert!(latest_block > 0);
    Ok(())
}

#[tokio::test]
/// **Unit Test**: Tests the `collect_order_events` function with predefined parameters
/// to ensure it retrieves order events successfully.
async fn unit_test_collect_order_events() -> Result<(), Box<dyn Error>> {
    let ws_rpc_url = format!(
        "{}{}",
        constants::MAINNET_WS_RPC_BASIC_URL,
        constants::TEST_INFURA_API_KEY
    );
    let contract_address = constants::DEFAULT_CONTRACT_ADDRESS;
    let from_block = 21041924;
    let to_block = 22094919;
    let chunk_size = 1_000_000;
    let event_type = constants::TAKEORDER_EVENT_NAME;

    // Call the `collect_order_events` function
    let result = collect_order_events(
        &ws_rpc_url,
        contract_address,
        from_block,
        to_block,
        chunk_size,
        event_type,
        "unit_test.csv",
    )
    .await;

    // Ensure the result is successful and the event set is not empty
    assert!(result.is_ok());

    // Verify the CSV file contents (expected event count = 41)
    assert!(verify_csv("unit_test.csv", 41));
    Ok(())
}

#[test]
/// **Fuzz Test**: Generates random input values and tests the `collect_order_events`
/// function with various combinations of parameters.
fn fuzz_test_collect_order_events() {
    let config = Config {
        cases: 10, // Number of test iterations (cases)
        ..Config::default()
    };
    let mut runner = TestRunner::new(config);

    // Define strategies for the random input values
    let from_block_strategy = 21000000u64..=21041924;
    let to_block_strategy = 21041924u64..=22094919;
    let event_type_strategy = prop_oneof![
        Just(constants::TAKEORDER_EVENT_NAME.to_string()),
        Just(constants::CLEAR_EVENT_NAME.to_string()),
        Just(constants::DEFAULT.to_string())
    ];

    // Define the combined strategy for fuzzing
    let strategy = (from_block_strategy, to_block_strategy, event_type_strategy);

    // Run the fuzz test with random values
    runner
        .run(&strategy, |(from_block, to_block, event_type)| {
            let ws_rpc_url = format!(
                "{}{}",
                constants::MAINNET_WS_RPC_BASIC_URL,
                constants::TEST_INFURA_API_KEY
            );
            let contract_address = constants::DEFAULT_CONTRACT_ADDRESS;
            let chunk_size = 1_000_000;

            println!(
                "Running test with from_block: {}, to_block: {}, chunk_size: {}, event_type: {:?}",
                from_block, to_block, chunk_size, event_type
            );

            // Call the function with random input values
            let result = tokio::task::block_in_place(move || {
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    collect_order_events(
                        &ws_rpc_url,
                        contract_address,
                        from_block,
                        to_block,
                        chunk_size,
                        &event_type,
                        "fuzz_test.csv",
                    )
                    .await
                })
            });

            // Ensure the result is successful
            assert!(result.is_ok());
            Ok(())
        })
        .unwrap();
}

#[test]
/// **Unit Test**: Tests the `write_to_csv` function by writing sample order events to a
/// CSV file and verifying its contents.
fn test_write_to_csv() -> Result<(), Box<dyn Error>> {
    // Setup a temporary file for testing
    let temp_file = NamedTempFile::new()?;

    // Prepare test data for order events
    let events = vec![OrderEvent {
        tx_origin: H160::from_slice(decode("abc123abc123abc123abc123abc123abc123abcd")?.as_slice()),
        event_type: "TakeOrderV2".to_string(),
        txn_hash: H256::from_slice(
            decode("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")?.as_slice(),
        ),
        timestamp: 1617912345,
    }];

    // Initialize the CSV
    initialize_csv(temp_file.path().to_str().unwrap())?;
    // Call the function under test to write events to the CSV
    write_to_csv(temp_file.path().to_str().unwrap(), &events)?;

    // Read the content of the temporary file
    let mut content = String::new();
    let mut file = temp_file.reopen()?;
    file.read_to_string(&mut content)?;

    // Assert that the CSV content is as expected
    let expected_content = "tx.origin,event type,txn hash,timestamp\n\
                          0xabc123abc123abc123abc123abc123abc123abcd,TakeOrderV2,0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef,1617912345\n";

    assert_eq!(content, expected_content);

    Ok(())
}
