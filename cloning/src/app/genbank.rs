use std::fs::File;
extern crate gb_io;

struct Location {
    loc_str: String
}

struct Feature {
    feature_type: String,
    location: Location,
    qualifiers: Vec<String>
}

struct Extra {
    content: String
}

pub struct Record {
    sequence: String,
    is_circular: bool,
    Features: Vec<Feature>,
    Extras: Vec<Extra>
}

impl Record {
    pub fn from_file(&self, input_file: File) -> Result<Record, &'static str> {
        Err("Error opening file.")
    }
}

#[cfg(test)]
mod seq_util_tests {
    use super::*;
    use gb_io::reader::SeqReader;
    use std::io;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn parse_gb_file() {
        let file_name = "/Users/tony/Documents/E/Coding/VSCode/Cloning/cloning/src/app/1.gbk";
        let stdout = io::stdout();
        
        let file = File::open(file_name).unwrap();
        for seq in SeqReader::new(file) {
            let seq = seq.unwrap();
            let () = seq;
        }
    }
}
