use crate::util_dependencies::Regex;

//list of regex expression
pub enum RegExp {
    HashCenterRightOnly,
    HashParameterLeftOnly,
    OutsideParenthesis,
    InsideParenthesis,
}

impl RegExp {
    pub fn check_reg_ex(&self) -> Result<regex::Regex, regex::Error> {
        match self {
            RegExp::HashCenterRightOnly => Regex::new(r"([!=<>]=?)(\d+)"),
            RegExp::HashParameterLeftOnly => Regex::new(r"(?i)(([a-zA-Z][0-9]?)+([_]?[a-zA-Z0-9]+)?)"),
            RegExp::OutsideParenthesis => Regex::new(r"\)([^)]+)\("),
            RegExp::InsideParenthesis => Regex::new(r"\(([^)]+)\)")
        }
    }
}
