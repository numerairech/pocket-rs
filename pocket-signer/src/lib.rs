extern crate ed25519_dalek;
extern crate hex;
extern crate rand_core;

use rand_core::OsRng;

use ed25519_dalek::*;

use pocket_utils::address_from_public_key;

pub struct KeyManager {
    keypair: Keypair,
    address: String,
}

impl KeyManager {
    pub fn new(private_key: &str) -> Self {
        let mut bytes = [0u8; 64];
        hex::decode_to_slice(private_key, &mut bytes).expect("Invalid private key");

        let keypair = Keypair::from_bytes(&bytes).unwrap();

        let address = address_from_public_key(&hex::encode(keypair.public.to_bytes())).unwrap();

        KeyManager { keypair, address }
    }

    pub fn new_from_slice(bytes: &[u8]) -> Self {
        let keypair = Keypair::from_bytes(bytes).unwrap();

        let address = address_from_public_key(&hex::encode(keypair.public.to_bytes())).unwrap();

        KeyManager { keypair, address }
    }

    pub fn new_from_random() -> Self {
        let keypair: Keypair = Keypair::generate(&mut OsRng);

        let address = address_from_public_key(&hex::encode(keypair.public.to_bytes())).unwrap();

        KeyManager { keypair, address }
    }

    pub fn get_private_key(&self) -> String {
        format!(
            "{}{}",
            hex::encode(self.keypair.secret.to_bytes()),
            hex::encode(self.keypair.public.to_bytes())
        )
    }

    pub fn get_public_key(&self) -> String {
        hex::encode(self.keypair.public.to_bytes())
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn sign(&self, message: String) -> String {
        self.keypair
            .sign(&hex::decode(message).unwrap())
            .to_string()
            .to_lowercase()
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

        let key_manager = KeyManager::new(&private_key);

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

        let mut bytes = [0u8; 64];
        hex::decode_to_slice(private_key.clone(), &mut bytes).unwrap();

        let key_manager = KeyManager::new_from_slice(&bytes);

        assert_eq!(key_manager.get_private_key(), private_key);
        assert_eq!(key_manager.get_public_key(), public_key);
        assert_eq!(key_manager.get_address(), address);
    }

    #[test]
    fn it_signs_message() {
        let private_key: String = "1f8cbde30ef5a9db0a5a9d5eb40536fc9defc318b8581d543808b7504e0902bcb243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let signed_message = "5d04dfc0d0e579d815f761b452c7d01e5f20a71b9fb66dbbeb1959cffed9da0a621ee06dfd11171757f9c9541768eaf59cce75ac4acc1ad122556ec26e166108";
        let key_manager = KeyManager::new(&private_key);

        assert_eq!(key_manager.sign("deadbeef".to_string()), signed_message)
    }
}
