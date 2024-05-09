use std::fs::File;

struct Feature {
    Start: u32,
    End: u32,
    IsForwardStrand: bool,
    FeatureType: String,
    FeatureNote: String
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
    fn from_file(&self, input_file:File) -> Record {
        Record {
            Sequence:String::from(""),
            Features:null,
            Extras: null
        }
    }
}