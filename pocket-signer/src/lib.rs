use pocket_utils;

pub struct KeyManager {
    private_key: String,
    public_key: String,
    address: String,
}

impl KeyManager {
    pub fn new(private_key: String) -> Self {
        let public_key = pocket_utils::public_key_from_private(private_key.clone()).unwrap();
        let address = pocket_utils::address_from_public_key(public_key.clone()).unwrap();
        KeyManager {
            private_key,
            public_key,
            address,
        }
    }

    pub fn get_private_key(&self) -> String {
        self.private_key.clone()
    }

    pub fn get_public_key(&self) -> String {
        self.public_key.clone()
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn sign(self, message: String) -> String {
        todo!("")
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
}
