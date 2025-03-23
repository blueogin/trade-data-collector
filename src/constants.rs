pub const DEFAULT_CONTRACT_ADDRESS: &str = "0x0ea6d458488d1cf51695e1d6e4744e6fb715d37c";
pub const TAKEORDER_EVENT_NAME: &str = "TakeOrderV2";
pub const CLEAR_EVENT_NAME: &str = "ClearV2";
pub const DEFAULT: &str = "DEFAULT";
pub const ABI_FILE_PATH: &str = "./IOrderBookV4.json";
pub const OUTPUT_FILE_PATH: &str = "order_events.csv";
pub const CSV_HEADER: [&str; 4] = ["tx.origin", "event type", "txn hash", "timestamp"];

pub const ETHERSCAN_BASIC_URL: &str = "https://api.etherscan.io";

pub const MAINNET_WS_RPC_BASIC_URL: &str = "wss://mainnet.infura.io/ws/v3/";
pub const BASE_WS_RPC_BASE_URL: &str = "wss://base-mainnet.infura.io/ws/v3/";
pub const ARBITRUM_WS_RPC_BASE_URL: &str = "wss://arbitrum-mainnet.infura.io/ws/v3/";
pub const OPTIMISM_WS_RPC_BASE_URL: &str = "wss://optimism-mainnet.infura.io/ws/v3/";
pub const LINEA_WS_RPC_BASE_URL: &str = "wss://linea-mainnet.infura.io/ws/v3/";

pub const TEST_INFURA_API_KEY: &str = "afee43fb439a4e1794d9acad3e4a95b8";
