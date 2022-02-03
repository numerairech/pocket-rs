pub mod utils {
    pub use pocket_utils::*;
}

#[cfg(test)]
mod tests {
    use crate::utils::get_gm;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_gets_gm() {
        let gm = get_gm();
        assert_eq!(gm, "gm");
    }
}
