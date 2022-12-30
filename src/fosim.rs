use clap::{CommandFactory, Parser, Subcommand};

/// A fast local Ethereum simulator based on top of foundry.
#[derive(Debug, Parser)]
#[clap(name = "fosim")]
pub struct App {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::parse();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Based from foundry's anvil: https://github.com/foundry-rs/foundry/blob/master/anvil/src/anvil.rs#L67
    #[test]
    fn can_parse_help() {
        let _: App = App::parse_from(["fosim", "--help"]);
    }
}
