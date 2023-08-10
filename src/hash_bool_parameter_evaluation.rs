use crate::util_dependencies::{Regex,LinkedHashMap};
use crate::regular_expression_list::RegExp;

// fn to print expression
pub fn print_for_expression(left:&i8, operator:&&str,right:&i8){
    println!("Ckecking for expression:{}{}{}", left, operator,right);
}

// Expressions to assign operator functions from Regex and String
pub fn evaluate_expression_for_bool(expression_operator: &str, 
    parameter_hash:&LinkedHashMap<String, i8>) -> bool {

    let reg_exp_assign = RegExp::HashCenterRightOnly.check_reg_ex().unwrap();
    let reg_exp_param = RegExp::HashParameterLeftOnly.check_reg_ex().unwrap();
    if let (Some(caps),Some(caps2)) = 
    (reg_exp_assign.captures(expression_operator),reg_exp_param.captures(expression_operator)) {

        let str_variable = caps2[1].to_string();
        let right: i8 = caps[2].parse::<i8>().unwrap();
        let operator = &caps[1];

        let left = if let Some(&value) = parameter_hash.get(&str_variable) { value } 
        else {
            println!("Key '{}' not found.", str_variable);
            panic!("Error: No key");
        };

        print_for_expression(&left,&operator, &right);
        match operator {
            "==" => left == right,
            "!=" => left != right,
            ">=" => left >= right,
            "<=" => left <= right,
            ">" => left > right,
            "<" => left < right,
            _ => false,
        }
    } else {
        panic!("Error: No capture, returning value as False");
        //false
    }
}


// Fn to get modules for regex to executable code inside parenthesis from string
pub fn create_parameter_bool_dictionary(list: &Regex, 
    expression_str: &str, 
    hash_bool: &mut LinkedHashMap<String, bool>,
    parameter_hash: &LinkedHashMap<String, i8>) {
    
    println!("");
    for cap in list.captures_iter(expression_str) {

        if let Some(capture) = cap.get(1) {

            let expression = capture.as_str();
            if !expression.is_empty() && !(expression=="&&" || expression=="||") {
                let result_expression = evaluate_expression_for_bool
                (expression,parameter_hash); 
                hash_bool.insert(expression.to_string(), result_expression);
            } else {
                panic!("Error in creating parameter bool. Check string"); 
            }
        }
    }
}
