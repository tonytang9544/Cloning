use std::fs::File;

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

struct Record {
    sequence: String,
    is_circular: bool,
    Features: Vec<Feature>,
    Extras: Vec<Extra>
}

impl Record {
    fn from_file(&self, input_file: File) -> Result<Record, &'static str> {
        Err("Error opening file.")
    }
}

#[cfg(test)]
mod seq_util_tests {
    use super::*;
}
