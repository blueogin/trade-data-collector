# Trade Data Collector

The **Trade Data Collector** is a Rust-based application designed to collect and export trade order events from a specified Ethereum-based decentralized exchange (DEX). It connects to the blockchain using a WebSocket RPC connection and listens for contract events, such as `TakeOrderV2` and `ClearV2`, that signify order actions on a DEX. The collected events are then written to a CSV file for further analysis.

## Features

- Connects to Ethereum networks using WebSocket RPC.
- Collects order-related events from smart contracts.
- Supports filtering events by type (`TakeOrderV2`, `ClearV2`, etc.).
- Exports collected events to a CSV file.
- Configurable network and contract address via command-line arguments.
- Supports multiple Ethereum networks (Mainnet, Arbitrum, Optimism, etc.).

## Prerequisites

- Rust 1.58 or higher.
- WebSocket RPC URL for the Ethereum network you want to connect to.
- Etherscan API key for fetching contract creation blocks (optional).
- ABI JSON file for the contract you want to track events from.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/trade-data-collector.git
    cd trade-data-collector
    ```

2. Set up environment variables:

    - Create a `.env` file in the project root directory by referencing `.env.example` file.

    - Replace `YOUR_INFURA_PROJECT_ID` with your Infura or Alchemy WebSocket RPC URL.

3. Install dependencies:

    ```bash
    cargo build
    ```

## Usage

Run the application with the desired options using the following command format:

```bash
cargo run -- --network <NETWORK> --contract <CONTRACT_ADDRESS> --event <EVENT_TYPE>
