#![feature(negative_impls)]
#[cfg(test)]
mod test {
    use deprive::deprive;

    #[deprive(Send, Sync)]
    struct X {}

    #[test]
    fn test_deprive() {
        let _x: X = X {};
    }
}
