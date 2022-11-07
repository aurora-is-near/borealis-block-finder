use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub refiner: Refiner,
    pub output_storage: OutputStoreConfig,
    pub input_mode: InputMode,
    pub initial_height: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Refiner {
    pub chain_id: u64,
    pub engine_path: String,
    pub engine_account_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OutputStoreConfig {
    /// Path to the folder where all blocks will be stored
    pub path: String,
    /// Number of files (blocks) to store on each folder.
    pub batch_size: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub enum InputMode {
    DataLake(DataLakeConfig),
    Nearcore(NearcoreConfig),
}

#[derive(Deserialize, Clone, Debug)]
pub struct DataLakeConfig {
    pub network: Network,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NearcoreConfig {
    pub path: String,
}

#[derive(Deserialize, Clone, Debug)]
pub enum Network {
    Mainnet,
    Testnet,
}
