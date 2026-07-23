use alloy::contract::Error as ContractError;
use alloy::sol_types::{Revert, SolError};

/// Alloy equivalent of the old ethers `parse_provider_error`: pulls the `Error(string)` revert
/// reason out of an alloy contract error and formats it as `execution reverted: <reason>`,
/// matching the ethers output so revert-string consumers (e.g. the sequencer's
/// `decode_gas_limit_revert` `": G ..."` gas protocol) keep working unchanged.
pub fn parse_alloy_error(error: &ContractError) -> String {
    if let Some(data) = error.as_revert_data() {
        if let Ok(revert) = Revert::abi_decode(data.as_ref()) {
            return format!("execution reverted: {}", revert.reason);
        }
        return "execution reverted: ".to_string();
    }
    error.to_string()
}
