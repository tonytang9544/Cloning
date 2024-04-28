pub fn Complement (seq: String) -> Result<String, &'static str> {
    let mut compl = String::new();
    for each_char in seq.chars() {
        compl.push(ComplementMap(each_char)?);
    }
    Ok(compl)
}

pub fn ReverseComplement (seq: String) -> Result<String, &'static str> {
    let compl = Complement(seq)?;
    Ok(compl.chars().rev().collect::<String>())
} 

fn ComplementMap (nuc: char) -> Result<char, &'static str> {
    match nuc {
        'A' => Ok('T'),
        'a' => Ok('t'),
        'T' => Ok('A'),
        't' => Ok('a'),
        'C' => Ok('G'),
        'c' => Ok('g'),
        'G' => Ok('C'),
        'g' => Ok('c'),
        _ => Err("Invalid nucleotide!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn complement() {
        assert_eq!(Complement(String::from("ATCGgcat")), Ok(String::from("TAGCcgta")));
        
    }

    #[test]
    fn reversecompl() {
        assert_eq!(ReverseComplement(String::from("ATCGgcat")), Ok(String::from("atgcCGAT")));
    }
}
