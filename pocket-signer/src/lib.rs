use ed25519_compact::{KeyPair, Seed};
use pocket_utils::address_from_public_key;

pub struct KeyManager {
    key_pair: KeyPair,
    address: String,
}

impl KeyManager {
    pub fn new(private_key: String) -> Self {
        let mut bytes = [0u8; 64];
        hex::decode_to_slice(private_key.clone(), &mut bytes).expect("Invalid private key");

        let key_pair = KeyPair::from_slice(&bytes).unwrap();

        let address = address_from_public_key(hex::encode(key_pair.pk.to_vec())).unwrap();

        KeyManager { key_pair, address }
    }

    pub fn new_from_slice(bytes: &[u8]) -> Self {
        let key_pair = KeyPair::from_slice(bytes).unwrap();

        let address = address_from_public_key(hex::encode(key_pair.pk.to_vec())).unwrap();

        KeyManager { key_pair, address }
    }

    pub fn get_private_key(&self) -> String {
        hex::encode(self.key_pair.sk.to_vec())
    }

    pub fn get_public_key(&self) -> String {
        hex::encode(self.key_pair.pk.to_vec())
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn sign(&self, message: String) -> String {
        hex::encode(self.key_pair.sk.sign(message, None).to_vec())
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn it_initializes_a_key_manager() {
        let private_key: String = "1f8cbde30ef5a9db0a5a9d5eb40536fc9defc318b8581d543808b7504e0902bcb243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let public_key: String =
            "b243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let address = "b50a6e20d3733fb89631ae32385b3c85c533c560".to_string();

        let key_manager = KeyManager::new(private_key.clone());

        assert_eq!(key_manager.get_private_key(), private_key);
        assert_eq!(key_manager.get_public_key(), public_key);
        assert_eq!(key_manager.get_address(), address);
    }

    #[test]
    fn it_initializes_a_key_manager_with_slice() {
        let private_key: String = "1f8cbde30ef5a9db0a5a9d5eb40536fc9defc318b8581d543808b7504e0902bcb243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let public_key: String =
            "b243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let address = "b50a6e20d3733fb89631ae32385b3c85c533c560".to_string();

        let mut out = [0u8; 64];
        hex::decode_to_slice(private_key.clone(), &mut out);

        let key_manager = KeyManager::new_from_slice(&out);

        assert_eq!(key_manager.get_private_key(), private_key);
        assert_eq!(key_manager.get_public_key(), public_key);
        assert_eq!(key_manager.get_address(), address);
    }

    #[test]
    fn it_signs_message() {
        let private_key: String = "1f8cbde30ef5a9db0a5a9d5eb40536fc9defc318b8581d543808b7504e0902bcb243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let signed_message = "628db0d268cc98fa56fcf2a307ff480b00911f6f9a71f10524d8fb4483230fdd9e857e194bd795977193bab4ea136dcfc09529fdbe8cab3a5e2106b5edd05109";
        let key_manager = KeyManager::new(private_key.clone());

        assert_eq!(key_manager.sign("deadbeef".to_string()), signed_message)
    }
}
