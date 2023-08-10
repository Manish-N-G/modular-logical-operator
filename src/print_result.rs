pub use crate::LogicalOp;

// fn to print result to terminal
pub fn print_list_result(cloned_values_exp: &Vec<bool>, cloned_values_op: &Vec<LogicalOp>){
    println!("\nExpression Bool List \t\t{:?}", cloned_values_exp);
    println!("\nLogical Operator List \t\t{:?}", cloned_values_op);
}

pub fn print_main_result(final_result: &bool, module_result: &Vec<bool>){
    println!("\nResult of Modules = {:?}",module_result);
    println!("\nResult of Expression = {:?}\n", final_result);

}
