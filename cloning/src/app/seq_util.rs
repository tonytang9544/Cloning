pub fn complement (seq: String) -> Result<String, &'static str> {
    let mut compl = String::new();
    for each_char in seq.chars() {
        compl.push(complement_map(each_char)?);
    }
    Ok(compl)
}

pub fn reverse_complement (seq: String) -> Result<String, &'static str> {
    let compl = complement(seq)?;
    Ok(compl.chars().rev().collect::<String>())
} 

fn complement_map (nuc: char) -> Result<char, &'static str> {
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
mod seq_util_tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_complement() {
        assert_eq!(complement(String::from("ATCGgcat")), Ok(String::from("TAGCcgta")));
        assert_eq!(complement(String::from("abcde")), Err("Invalid nucleotide!"));
    }

    #[test]
    fn test_reversecompl() {
        assert_eq!(reverse_complement(String::from("ATCGgcat")), Ok(String::from("atgcCGAT")));
    }
}
