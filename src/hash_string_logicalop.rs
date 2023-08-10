use crate::logicalop_initialisation::{LogicalOp, evaluate_expression_for_logicalop};
use crate::util_dependencies::{LinkedHashMap,Regex};

// Fn to get modules for regex to executable code to identify logical Exp from string
pub fn create_hash_for_logiclop_dictionary(list: &Regex, 
    expression_str: &str, 
    hash_logic: &mut LinkedHashMap<String, LogicalOp>) {

    let mut counter = 0;
    for cap in list.captures_iter(expression_str) {

        if let Some(capture) = cap.get(1) {
            let expression = capture.as_str();

            if expression == "&&" || expression == "||" {
                counter = counter + 1;
                let result_logic = evaluate_expression_for_logicalop(expression);
                hash_logic.insert(counter.to_string(), result_logic);
            } else {
                panic!("Error found in evaluation for Logicalop");  
            }
        }
    }
}
