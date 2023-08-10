//#![allow(unused)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C"{
    fn alert(s: &str);
}

// list of modules for lib
pub mod util_dependencies;
pub mod user_inputs;
pub mod user_parameters;
pub mod logicalop_initialisation;
pub mod hash_string_logicalop;
pub mod hash_bool_parameter_evaluation;
pub mod evaluate_bool_result;
pub mod compute_final_results;
pub mod print_result;
pub mod regular_expression_list;

// Required of defined types and dependencies inside lib
pub use crate::logicalop_initialisation::LogicalOp as LogicalOp;
use crate::util_dependencies::LinkedHashMap;
use crate::regular_expression_list::RegExp;



#[wasm_bindgen]
pub fn main_logic_function() {

    // *****         user input block             ******//
    let binding = user_inputs::input_expression();
    let variables_expression = binding.as_str();
    let if_confirm = user_inputs::confirm_user_expression();
    let expression_string = user_inputs::check_expression(if_confirm, variables_expression);
    let expression_str = expression_string.as_str();
    //Incorrest eg >>    (5==6)&&(7==7)||(7>=6)&&(2>8)||(4!=5)
    //Correct eg >>  (a1a_a1==6)&&(b_==7)||(c_22>=6)&&(dd>88)||(e!=5)||(f<=2)


    // *****         user parameter block             ******//
    let reg_exp_for_parameter = RegExp::HashParameterLeftOnly.check_reg_ex().unwrap();
    let parameter_hash = 
    user_parameters::assign_user_parameters_hash(reg_exp_for_parameter,expression_str);


    // *****         create hash block             ******//
    let regular_exp_factors = RegExp::OutsideParenthesis.check_reg_ex().unwrap();
    let regular_exp_parentesis = RegExp::InsideParenthesis.check_reg_ex().unwrap();
    let mut logical_hash: LinkedHashMap<String, LogicalOp> = LinkedHashMap::new();
    let mut bool_hash: LinkedHashMap<String, bool> = LinkedHashMap::new();
    // fn to assess the expressions for Logicical Operators-op and Conditional Operators-exp
    hash_string_logicalop::create_hash_for_logiclop_dictionary(
        &regular_exp_factors, expression_str, &mut logical_hash);

    hash_bool_parameter_evaluation::create_parameter_bool_dictionary(
        &regular_exp_parentesis, expression_str, &mut bool_hash,&parameter_hash);
 

    // *****         compute result block             ******//
    println!("\n\n\t\t---Computing Results---\t\t\n");
    let (main_result,
        modules_result,
        expression_mod_bool_list,
        expression_logical_list)=
        compute_final_results::compute_main_results(&parameter_hash, &bool_hash, &logical_hash );


    // *****         print result block             ******//
    println!("\n\n\t\t---Final Results---\t\t\n");
    print_result::print_list_result(&expression_mod_bool_list, &expression_logical_list);
    print_result::print_main_result(&main_result, &modules_result);

}
