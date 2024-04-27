

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn complement() {
        assert_eq!(crate::app::SeqUtil::Complement(String::from("ATCGgcat")), Ok(String::from("TAGCcgta")));
    }

    #[test]
    fn reversecompl() {
        assert_eq!(crate::app::SeqUtil::ReverseComplement(String::from("ATCGgcat")), Ok(String::from("atgcCGAT")));
    }
}