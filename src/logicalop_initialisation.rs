use std::fmt;
//Values for Initialising LogicalOp for AND and OR
#[derive(Debug, Clone)]
pub enum LogicalOp {
    AND,
    OR
}

// Implementation method to match Display with "AND"/"OR" with Logical AND/OR. Its to display LogicalOp
impl fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogicalOp::AND => write!(f, "AND"),
            LogicalOp::OR => write!(f, "OR"),
        }
    }
}

// Evaluate string values with match str with Logic
pub fn evaluate_expression_for_logicalop(expression: &str) -> LogicalOp {
    match expression {
        "||" => LogicalOp::OR,
        "&&" => LogicalOp::AND,
        _ => panic!("Invalid logical operator"),
    }
}
