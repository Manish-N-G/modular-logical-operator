use crate::util_dependencies::LinkedHashMap;
use crate::LogicalOp;

// fn to print calculation
pub fn print_bool_logic_calculation(cloned_exp:&Vec<bool>,
    cloned_op:&Vec<LogicalOp>, res:&bool, i:usize){
    println!("{}\t {}\t {}\t = \t{}",
        cloned_exp[i], cloned_op[i], cloned_exp[i+1], res
    );
}

// fn to calculate from vector lists to produce result
pub fn result_calculate(cloned_values_exp:&Vec<bool>, cloned_values_op:&Vec<LogicalOp>) -> (Option<bool>, Vec<bool>){
    let mut result = None;
    let mut module_result: Vec<bool> = Vec::new();

    if cloned_values_exp.len() == cloned_values_op.len() + 1 {
        for i in 0..cloned_values_op.len() {
            let res = match cloned_values_op[i] {
                LogicalOp::AND => cloned_values_exp[i] && cloned_values_exp[i + 1], // Perform logical AND operation
                LogicalOp::OR => cloned_values_exp[i] || cloned_values_exp[i + 1],  // Perform logical OR operation
            };

            module_result.push(res);
            result = Some(res);

            print_bool_logic_calculation(&cloned_values_exp, 
            &cloned_values_op,&res, i);
        }
    } else {
        panic!("Error: In Lists lengths, Check Expression");
    }
    (result,module_result)
}


// Fn to compute logical and boolean expression through list
pub fn compute_boolean_result(
    dict_bool: &LinkedHashMap<String, bool>,
    dict_logic: &LinkedHashMap<String, LogicalOp>,
) -> (Option<bool>, Vec<bool>, Vec<bool>, Vec<LogicalOp>) {

    let mut cloned_values_exp: Vec<bool> = Vec::new();
    let mut cloned_values_op: Vec<LogicalOp> = Vec::new();

    for (_, value) in dict_bool {
        cloned_values_exp.push(*value);
    }

    for (_, value) in dict_logic {
        cloned_values_op.push(value.clone());
    }
    
    let (result_tuple, module_tuple) = 
    result_calculate(&cloned_values_exp, &cloned_values_op);

    (result_tuple, module_tuple,cloned_values_exp,cloned_values_op)

}
