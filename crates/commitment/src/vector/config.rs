use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde_as(as = "UfeHex")]
    height: Felt,
    #[serde_as(as = "UfeHex")]
    n_verifier_friendly_commitment_layers: Felt,
}

impl Config {
    pub fn validate(
        &self,
        expected_height: Felt,
        expected_n_verifier_friendly_commitment_layers: Felt,
    ) -> Result<(), Error> {
        if self.height != expected_height {
            return Err(Error::MisMatch { value: self.height, expected: expected_height });
        }
        if self.n_verifier_friendly_commitment_layers
            != expected_n_verifier_friendly_commitment_layers
        {
            return Err(Error::MisMatch {
                value: self.n_verifier_friendly_commitment_layers,
                expected: expected_n_verifier_friendly_commitment_layers,
            });
        }

        Ok(())
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mismatch value {value} expected {expected}")]
    MisMatch { value: Felt, expected: Felt },
}
