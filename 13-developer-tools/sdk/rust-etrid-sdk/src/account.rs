//! Account management and key operations

use sp_core::{sr25519, Pair, crypto::Ss58Codec};
use crate::{Error, Result};

/// Ëtrid account
pub struct Account {
    pair: sr25519::Pair,
}

impl Account {
    /// Create account from mnemonic phrase
    ///
    /// # Example
    ///
    /// ```
    /// # use etrid_sdk::Account;
    /// let account = Account::from_mnemonic("word1 word2 ... word12").unwrap();
    /// ```
    pub fn from_mnemonic(mnemonic: &str) -> Result<Self> {
        let pair = sr25519::Pair::from_phrase(mnemonic, None)
            .map_err(|e| Error::Crypto(format!("Invalid mnemonic: {:?}", e)))?
            .0;

        Ok(Self { pair })
    }

    /// Generate a new random account
    pub fn generate() -> Self {
        let (pair, _seed, _) = sr25519::Pair::generate_with_phrase(None);
        Self { pair }
    }

    /// Get the account address (SS58 format)
    pub fn address(&self) -> String {
        self.pair.public().to_ss58check()
    }

    /// Get the public key (hex format)
    pub fn public_key(&self) -> String {
        format!("0x{}", hex::encode(self.pair.public().as_ref()))
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.pair.sign(message).0.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_generate() {
        let account = Account::generate();
        assert!(!account.address().is_empty());
    }

    #[test]
    fn test_account_from_mnemonic() {
        let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
        let account = Account::from_mnemonic(mnemonic);
        assert!(account.is_ok());
    }
}
