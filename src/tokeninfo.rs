#[derive(Clone, Debug, PartialEq)]
pub struct TokenInfo {
    pub litertal: String,
    pub line: usize,
    pub col: usize,
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo {
            litertal: "".into(),
            line: 0,
            col: 0,
        }
    }
}
