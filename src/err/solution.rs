use std::error::Error;

#[derive(Debug)]
pub enum SolutionError {
    ParseError(&'static str, String),
    NoAnswer
}

impl Error for SolutionError {}

impl std::fmt::Display for SolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Solution finding failed {}",
            match self {
                Self::ParseError(m, input) => format!("Parse error {} on {}", m, input),
                Self::NoAnswer => "No answer found!".to_owned()
            }
        )
    }
}
