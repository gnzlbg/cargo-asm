pub mod ast;
pub mod parse;

#[derive(Copy, Clone, Debug)]
pub enum Style {
    Intel,
    ATT,
}

impl ::std::str::FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "intel" => Ok(Style::Intel),
            "at&t" => Ok(Style::ATT),
            v => Err(format!("\"{}\" is not a valid assembly style. Try \"intel\" or \"at&t\"", v))
        }
    }
}
