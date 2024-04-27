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
    Sequences: String,
    Features: Vec<Feature>,
    Extras: Vec<Extra>
}
