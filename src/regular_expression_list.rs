use crate::util_dependencies::Regex;
//use crate::util_dependencies::RegExp;

//list of regex expression
pub enum RegExpEnum {
    HashCenterRightOnly,
    HashParameterLeftOnly,
    OutsideParenthesis,
    InsideParenthesis,
}

impl RegExpEnum {
    pub fn check_reg_ex(&self) -> Result<Regex, regex::Error> {
        match self {
            //RegExpEnum::HashCenterRightOnly => RegExp::new(r"([!=<>]=?)(\d+)", "asd"),
            //RegExpEnum::HashParameterLeftOnly => RegExp::new(r"(?i)(([a-zA-Z][0-9]?)+([_]?[a-zA-Z0-9]+)?)", "asd"),
            //RegExpEnum::OutsideParenthesis => RegExp::new(r"\)([^)]+)\(", "asd"),
            //RegExpEnum::InsideParenthesis => RegExp::new(r"\(([^)]+)\)", "asd")
            RegExpEnum::HashCenterRightOnly => Regex::new(r"([!=<>]=?)(\d+)"),
            RegExpEnum::HashParameterLeftOnly => Regex::new(r"(?i)(([a-zA-Z][0-9]?)+([_]?[a-zA-Z0-9]+)?)"),
            RegExpEnum::OutsideParenthesis => Regex::new(r"\)([^)]+)\("),
            RegExpEnum::InsideParenthesis => Regex::new(r"\(([^)]+)\)")
        }
    }
}

