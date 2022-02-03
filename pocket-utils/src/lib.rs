pub fn gm() {
    println!("gm");
}

pub fn get_gm() -> &'static str {
    "gm"
}

#[cfg(test)]
mod tests {
    use crate::get_gm;

    #[test]
    fn it_gets_gm() {
        let gm = get_gm();
        assert_eq!(gm, "gm");
    }
}
