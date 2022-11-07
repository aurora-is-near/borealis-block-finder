use clap::{Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {

    /// [Optional] Path to config file.
    /// If this value is not passed the default config will be used.
    #[clap(short, long)]
    pub config_path: Option<String>,

    /// [Optional] Start refiner from specified height.
    /// If this value is not passed, first height will be used.
    #[clap(short = 'n', long)]
    pub height: Option<u64>,

    /// [Optional] Near block expression to match.
    /// If this value is not passed it will not be used to find matches.
    #[clap(short, long)]
    pub near_block_expression: Option<String>,

    /// [Optional] Aurora block expression to match.
    /// If this value is not passed it will not be used to find matches.
    #[clap(short, long)]
    pub aurora_block_expression: Option<String>,

    // ToDo:
    // Not sure about the 'short' and 'long' parameters here.
    // Parameter height default value explanation.
}
