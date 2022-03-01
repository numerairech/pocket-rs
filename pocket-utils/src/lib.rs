pub fn public_key_from_private(private_key: String) -> String {
    private_key.chars().skip(64).collect()
}

#[cfg(test)]
mod tests {
    use crate::public_key_from_private;

    #[test]
    fn can_get_public_key() {
        let private_key: String = "1f8cbde30ef5a9db0a5a9d5eb40536fc9defc318b8581d543808b7504e0902bcb243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        let public_key: String =
            "b243b27bc9fbe5580457a46370ae5f03a6f6753633e51efdaf2cf534fdc26cc3".to_string();
        assert_eq!(public_key_from_private(private_key), public_key);
    }
}
