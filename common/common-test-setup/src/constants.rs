use dharitri_sc_scenario::imports::{DrtscPath, TestAddress, TestSCAddress};

pub const DCDT_SAFE_ADDRESS: TestSCAddress = TestSCAddress::new("dcdt-safe");
pub const FEE_MARKET_ADDRESS: TestSCAddress = TestSCAddress::new("fee-market");
pub const HEADER_VERIFIER_ADDRESS: TestSCAddress = TestSCAddress::new("header-verifier");

pub const CHAIN_CONFIG_ADDRESS: TestSCAddress = TestSCAddress::new("chain-config");

pub const TESTING_SC_ADDRESS: TestSCAddress = TestSCAddress::new("testing-sc");
pub const ENSHRINE_ADDRESS: TestAddress = TestAddress::new("enshrine");

pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
pub const USER: TestAddress = TestAddress::new("user");

pub const FEE_MARKET_CODE_PATH: DrtscPath =
    DrtscPath::new("../fee-market/output/fee-market.drtsc.json");
pub const HEADER_VERIFIER_CODE_PATH: DrtscPath =
    DrtscPath::new("../header-verifier/output/header-verifier.drtsc.json");
pub const CHAIN_CONFIG_CODE_PATH: DrtscPath =
    DrtscPath::new("../chain-config/output/chain-config.drtsc.json");
pub const TESTING_SC_CODE_PATH: DrtscPath =
    DrtscPath::new("../testing-sc/output/testing-sc.drtsc.json");
pub const DRT_DCDT_SAFE_CODE_PATH: DrtscPath =
    DrtscPath::new("../drt-dcdt-safe/output/drt-dcdt-safe.drtsc.json");
pub const SOV_DCDT_SAFE_CODE_PATH: DrtscPath =
    DrtscPath::new("../sov-dcdt-safe/output/to-sovereign.drtsc.json");

pub const FEE_TOKEN: &str = "INTERNS-eaad15";
pub const FIRST_TEST_TOKEN: &str = "GREEN-0e161c";
pub const SECOND_TEST_TOKEN: &str = "LTST-4f849e";
pub const SOV_TOKEN: &str = "sov-GREEN-0e161c";
pub const TOKEN_TICKER: &str = "GREEN";

pub const SOV_TO_DRT_TOKEN_STORAGE_KEY: &str = "sovToMxTokenId";
pub const DRT_TO_SOV_TOKEN_STORAGE_KEY: &str = "drtToSovTokenId";
pub const OPERATION_HASH_STATUS_STORAGE_KEY: &str = "operationHashStatus";

pub const ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 REWA
pub const ONE_HUNDRED_MILLION: u32 = 100_000_000;
pub const ONE_HUNDRED_THOUSAND: u32 = 100_000;
pub const OWNER_BALANCE: u128 = 100_000_000_000_000_000_000_000;
