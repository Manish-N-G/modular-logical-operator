#![allow(unused)]
//use logical_module_program::*; //works for all function
use logical_module_program::main_logic_function;
use logical_module_program::user_inputs;
use logical_module_program::user_parameters;
use logical_module_program::hash_string_logicalop::*;
use logical_module_program::hash_bool_parameter_evaluation::*;
use logical_module_program::evaluate_bool_result::*;
use logical_module_program::print_result::*;
use logical_module_program::compute_final_results;
use logical_module_program::user_parameters::*;
use logical_module_program::js_user_parameters::*;


use regex::Regex;
use linked_hash_map::LinkedHashMap;


// Simple module to remove all white spaces in string
#[cfg(test)]
mod test_user_input_whitespace{
    use super::*;
    #[test]
    fn whitespaces_test() {
        let whitespace = 
        user_inputs::remove_whitespace("( 5 = =6)& & ( 7==7)|| (7 >=   6)");
        println!("For whitespace: {}",whitespace);
    }

}



// Simple module to print types expression without to confirm no whitespaces
//#[cfg(test)]
mod test_user_input_type{
    use super::*;

    //#[test] //manual testing
    fn input_string_test() {
        println!("\nUser Input:Type this expression of type String  :\nType eg (a==4)||(b!=3)&&(c<=3)\n");
        let string_input = user_inputs::input_expression();
        println!("For string_input: {}",string_input);
        // pass input1 eg ( 5 = =6)& & ( 7==7)|| (7 >=   6)
        // pass input2 eg ( 5 = =6)& & ( 7==7)|| (7 >=   6
        // pass input3 eg ( x = 6)& & ( a==7)  || (b >=   6)
        // pass input4 eg (阿 ==4 ) || (斯达 !=2) && (斯达==广告)
        assert!(true);
    }

}



// Module to confirm choice of user inputted string
//#[cfg(test)]
mod test_user_input_choice_bool{
    use super::*;

    //#[test] //manual testing
    fn confirm_choice_test() {
        println!("\nUser Input:Type this expression of type String  :\nType eg t/t\n");
        let bool_val = user_inputs::confirm_user_expression();
        println!("For bool_val: {}",bool_val);
        // confirm for t, f, T, F
    }
}



// Module to allow for change of input string
//#[cfg(test)]
mod test_user_input_choice_change{
    use super::*;

    //#[test] //manual testing
    fn change_choice_test(){
        let exp = "(a1a1_a1==6)&&(b_==7)||(c_222>=6)";
        println!("\nUser Input:To retype String  :\nType eg (a==4)||(b!=3)&&(c<=3)\n");
        //let x = user_inputs::check_expression(true, exp);
        let y = user_inputs::check_expression(false, exp);
        // pass eg (a1a1_a1==6)&&(b_==7)||(c_222>=6)
        println!("For y::false, string: {}",y);
        //println!("For x::true, string: {}",x);
    }

}



// Module to ensure the imput parameter it type i8 integer
//#[cfg(test)]
mod test_user_input_parameters_int{
    use super::*;

    //#[test] //manual testing
    fn check_if_int_test() {
        println!("\nUser Input:Type integer:\nType eg 4 ,10, 0\n");
        let value = user_parameters::check_if_int("var_1");
        println!("For value: {}",value);
        // check is var_1 is int of i8
    }

}



// Module to assign parameters from user inputted string
//#[cfg(test)]
mod test_user_input_parameters_assign{
    use super::*;

    #[test] //manual testing
    fn ckeck_user_assigned_parameters_test() {
        println!("\nUser Input:Enter integer");
        let exp = "(a1a1_a1==6)&&(b_==7)||(c_222>=6)&&(ddd>88)||(ddd>88)";
        // change exp for different str types
        let reg_exp = Regex::new(r"(?i)(([a-zA-Z][0-9]?)+([_]?[a-zA-Z0-9]+)?)").unwrap();
        let para_hash = user_parameters::assign_user_parameters_hash(reg_exp,exp);
        //let para_hash = user_parameters::assign_user_parameters_hash2(r"(?i)(([a-zA-Z][0-9]?)+([_]?[a-zA-Z0-9]+)?)",exp);
        println!("For parameter hash: {:?}",para_hash);
    }
}





// Simple module to see if no panic for string accepted for Logical hash creation
#[cfg(test)]
mod test_string_logicalop{
    use super::*;
    
    #[test]
    fn check_logicalop_created_test() {
        let exp = "(a1a1_a1==6)&&(b_==7)||(c_222>=6)&&(ddd>88)";
        // change exp for different str types
        let mut logical_hash: LinkedHashMap<String, LogicalOp> = LinkedHashMap::new();
        let reg_exp = Regex::new(r"\)([^)]+)\(").unwrap();
        create_hash_for_logiclop_dictionary(
            &reg_exp,exp, &mut logical_hash);
    }
}



 
// Module to produce frue or false if parameter matched expression operation
#[cfg(test)]
mod test_parameter_evaluation_bool{
    use super::*;

    #[test]
    fn check_bool_evaluation_test() {
        let exp = vec!["a1a1_a1==6", "b_==7", "c_222>=6", "ddd>88"];
        // change exp for different str types
        let mut para_hash: LinkedHashMap<String, f64> =LinkedHashMap::new();
        // Manually insert key-value pairs
        para_hash.insert("a1a1_a1".to_string(), 6.0);
        para_hash.insert("b".to_string(), 5.0);
        para_hash.insert("c_222".to_string(), 3.0);
        para_hash.insert("ddd".to_string(), 2.0);

        for str in exp{
            let para_bool = evaluate_expression_for_bool(
                &str, &mut para_hash);
            println!("For hash para_bool {:?}", para_bool);
        } 
    }
}




// Module creates bool for variable parameters matching string expression
#[cfg(test)]
mod test_parameter_evaluation_create{
    use super::*;

    #[test]
    fn check_parameter_hash_created_test() {
        let exp = "(a1a1_a1==6)&&(b_==7)||(c_222>=6)&&(ddd>88)";
        // change exp for different str types
        let mut para_hash: LinkedHashMap<String, f64> =LinkedHashMap::new();
        // Manually insert key-value pairs
        para_hash.insert("a1a1_a1".to_string(), 5.0);
        para_hash.insert("b".to_string(), 6.0);
        para_hash.insert("c_222".to_string(), 3.0);
        para_hash.insert("ddd".to_string(), 2.0);

        let mut bool_hash: LinkedHashMap<String, bool> = LinkedHashMap::new();

        let reg_exp = Regex::new(r"\(([^)]+)\)").unwrap();
        create_parameter_bool_dictionary(&reg_exp,
            exp, &mut bool_hash, &para_hash);
        println!("\nFor bool_hash: {:?}\n",bool_hash);
    }
}




// Module to calculate for module 1 step at a time
#[cfg(test)]
mod test_evaluation_final_calculation{
    use super::*;
 
    #[test]
    fn result_calculation_test() {
        let exp_bool_array = vec![true, false, true, false];
        let op_bool_array = vec![LogicalOp::AND, LogicalOp::OR, LogicalOp::AND];
    
        if let (Some(result_tuple), module_tuple) =
            result_calculate(&exp_bool_array, &op_bool_array){

            println!("Module Tuple: {:?}", module_tuple);
            println!("Result Tuple: {:?}", result_tuple);
            }
    }
}



// Static module to create list from resulting bool value done in modules
#[cfg(test)]
mod test_evaluation_final_bool_create{
    use super::*;

    #[test]
    fn check_logicalop_created_test() {

        let mut bool_hash: LinkedHashMap<String, bool> =LinkedHashMap::new();
        // Manually insert key-value pairs
        bool_hash.insert("a1a1_a1".to_string(), true);
        bool_hash.insert("b".to_string(), false);
        bool_hash.insert("c_222".to_string(), true);
        bool_hash.insert("ddd".to_string(), false);
        // change exp for different str types

        let mut logical_hash: LinkedHashMap<String, LogicalOp> = LinkedHashMap::new();
        // Manually insert key-value pairs
        logical_hash.insert("1".to_string(), LogicalOp::AND);
        logical_hash.insert("2".to_string(), LogicalOp::OR);
        logical_hash.insert("3".to_string(), LogicalOp::AND);

        let mut para_hash: LinkedHashMap<String, i8> =LinkedHashMap::new();
        // where we assign eg a1a1_a1=6 ,b=2, c_222=7, ddd=3
        para_hash.insert("a1a1_a1".to_string(), 6);
        para_hash.insert("b".to_string(), 2);
        para_hash.insert("c_222".to_string(), 7);
        para_hash.insert("ddd".to_string(), 3);


        if para_hash.len()==bool_hash.len() && (para_hash.len()==logical_hash.len()+1){
            if let (Some(final_result),
                final_module,
                module_exp,
                module_op) =
                compute_boolean_result(  &bool_hash, &logical_hash){
                    println!("For module_exp: {:?}",module_exp);
                    println!("For module_op: {:?}",module_op);
                    println!("For final_module: {:?}",final_module);
                    println!("For final_result: {}",final_result);
            }
        }
    }
}




// Module to test if results are same as static pretermined values
#[cfg(test)]
mod test_evaluation_final_bool{
    use super::*;

    #[test]
    fn result_final_test() {

        let mut bool_hash: LinkedHashMap<String, bool> =LinkedHashMap::new();
        // Manually insert key-value pairs
        bool_hash.insert("a1a1_a1".to_string(), true);
        bool_hash.insert("b".to_string(), false);
        bool_hash.insert("c_222".to_string(), true);
        bool_hash.insert("ddd".to_string(), false);
        // change exp for different str types

        let mut logical_hash: LinkedHashMap<String, LogicalOp> = LinkedHashMap::new();
        // Manually insert key-value pairs
        logical_hash.insert("1".to_string(), LogicalOp::AND);
        logical_hash.insert("2".to_string(), LogicalOp::OR);
        logical_hash.insert("3".to_string(), LogicalOp::AND);

        let mut para_hash: LinkedHashMap<String, f64> =LinkedHashMap::new();
        // where we assign eg a1a1_a1=6 ,b=2, c_222=7, ddd=3
        para_hash.insert("a1a1_a1".to_string(), 6.0);
        para_hash.insert("b".to_string(), 2.0);
        para_hash.insert("c_222".to_string(), 7.0);
        para_hash.insert("ddd".to_string(), 3.0);


        let (main_result,
            modules_result,
            expression_mod_bool_list,
            expression_logical_list)=
            compute_final_results::compute_main_results(&para_hash, &bool_hash, &logical_hash );

            println!("\nConfirming for expression \n\nEesult: {},\nModule_result: {:?},\nSingle_Modules: {:?},list_LogicalOP: {:?}\n",
            main_result, modules_result, expression_mod_bool_list, expression_logical_list);

    }
}




// Module to print results to terminal it resired
#[cfg(test)]
mod test_print_module{
    use super::*;
 
    #[test]
    fn print_module_list_test() {
        let exp_bool_array: Vec<bool> = vec![true, false, true, false];
        let op_bool_array: Vec<LogicalOp> = vec![LogicalOp::AND, LogicalOp::OR, LogicalOp::AND];
    
        print_list_result(&exp_bool_array, &op_bool_array);
    }

    #[test]
    fn print_result_test() {
        let mod_result_array: Vec<bool> = vec![true, false, true, false];
        let result_of_expression: bool = true;
    
        print_main_result(&result_of_expression, &mod_result_array);
    }
}





