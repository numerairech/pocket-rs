use sha2::{Digest, Sha256};

pub mod errors;
use errors::Error;

/// Computes the address of an ed25519 public key.
pub fn address_from_public_key(public_key: &str) -> Result<String, Error> {
    let decoded_public_key = hex::decode(public_key);
    let decoded_public_key = match decoded_public_key {
        Ok(pk) => pk,
        Err(e) => match e {
            hex::FromHexError::OddLength => return Err(Error::InvalidPublicKeyLength),
            hex::FromHexError::InvalidStringLength => return Err(Error::InvalidPublicKeyLength),
            hex::FromHexError::InvalidHexCharacter { .. } => {
                return Err(Error::InvalidPublicKeyFormat)
            }
        },
    };

    let result = Sha256::digest(decoded_public_key);

    let address = hex::encode(result);

    Ok(address.to_string().chars().take(40).collect())
}

/// Extracts the public key from a 64-byte long ed25519 private key.
///
/// The public key from a ed25519 private key will always be the last 64 characters.
pub fn public_key_from_private(private_key: &str) -> Result<String, Error> {
    match private_key.len() {
        128 => Ok(private_key.chars().skip(64).collect()),
        _ => return Err(Error::InvalidPrivateKeyLength),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn can_get_public_key() {
        let private_key: String = "1f8cbde30ef5a9db0a5a9d5eb40536fc9defc318b8581d543808b7504e0902bcb243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let public_key: String =
            "b243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        assert_eq!(public_key_from_private(&private_key).unwrap(), public_key);
    }

    #[test]
    fn errors_with_invalid_len() {
        let private_key: String =
            "b243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string(); // First 64 characters removed
        assert!(public_key_from_private(&private_key).is_err());
    }

    #[test]
    fn can_get_address_from_public_key() {
        let public_key: String =
            "b243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let address = "b50a6e20d3733fb89631ae32385b3c85c533c560".to_string();
        assert_eq!(address_from_public_key(&public_key).unwrap(), address);
    }
}
