#[derive(Debug, Clone, PartialEq, Hash)]
pub struct UpfDocument;

#[cfg(test)]
mod tests {
    use super::UpfDocument;

    #[test]
    fn it_works() {
        println!("{:?}", UpfDocument);
        assert!(true);
    }
}
