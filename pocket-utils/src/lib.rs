use sha2::{Digest, Sha256};

/// Computes the address of an ed25519 public key.
pub fn address_from_public_key(public_key: String) -> String {
    let result = Sha256::digest(hex::decode(public_key).expect("Invalid public key"));
    let mut buf = [0u8; 64];

    let address = base16ct::lower::encode_str(&result, &mut buf).unwrap();

    address.to_string().chars().take(40).collect()
}

/// Extracts the public key from a 64-byte long ed25519 private key.
///
/// The public key from a ed25519 private key will always be the last 64 characters.
pub fn public_key_from_private(private_key: String) -> String {
    private_key.chars().skip(64).collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn can_get_public_key() {
        let private_key: String = "1f8cbde30ef5a9db0a5a9d5eb40536fc9defc318b8581d543808b7504e0902bcb243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let public_key: String =
            "b243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        assert_eq!(public_key_from_private(private_key), public_key);
    }

    #[test]
    fn can_get_address_from_public_key() {
        let public_key: String =
            "b243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let address = "b50a6e20d3733fb89631ae32385b3c85c533c560".to_string();
        assert_eq!(address_from_public_key(public_key), address);
    }
}
