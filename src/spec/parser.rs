pub struct Parser {
    filename: String,
}

impl Parser {
    /// If specify `stream = true`, this method will use buffer refilling instead of references to generate token.
    /// Performance may be affected.
    pub fn parse(filename: &str, buffer: bool)->bool;
}