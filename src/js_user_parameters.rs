#[allow(unused)]
use wasm_bindgen::prelude::*;
use crate::util_dependencies::*;
use crate::hash_string_logicalop;
use crate::hash_bool_parameter_evaluation;
use crate::user_inputs::remove_whitespace;
use crate::regular_expression_list::RegExpEnum;
use crate::compute_final_results;
use crate::LogicalOp;

use js_sys::{Array, Reflect, Object};
use crate::js_user_parameters::to_js_value::ToJsValue;


// user input for client side in js
#[wasm_bindgen]
pub fn enter_input(input: &str) -> String {
    let user_input = input.trim();
    user_input.to_string()
}

// fn to enter and validate for string for expression in js
#[wasm_bindgen]
pub fn input_expression_client(input:&str) -> bool {
        let input_str = remove_whitespace(input);
        if input_str.is_empty(){
            return false 
        } else {
            return true
        };
}





// creates variable names from input string in js for Leftonly
pub fn assign_user_leftparameters_set_js(list:Regex, expx:&str) ->Vec<&str> {
    
    let mut int_set : Vec<&str> = Vec::new();
    for cap in list.captures_iter(expx) { 
        if let Some(capture) = cap.get(1) {
            let expression = capture.as_str();
            int_set.push(expression);
        }
    }
    int_set
}

// function to count the number of variables for js
#[wasm_bindgen]
pub fn count_my_input_param_js(input:&str)-> i8{
    let list = RegExpEnum::HashParameterLeftOnly.check_reg_ex().unwrap();
    let my_array: Vec<&str> = assign_user_leftparameters_set_js(list, input);
    my_array.len() as i8
}


//creates a hash and assigns input array to variables from parameter_input for js
pub fn assign_user_parameters_js(list: Regex, array: Vec<f64>, expx: &str) -> LinkedHashMap<String, f64> {
    
    let mut int_hash: LinkedHashMap<String, f64> = LinkedHashMap::new();
    let mut counter: usize = 0;
    for cap in list.captures_iter(expx) {
        if let Some(capture) = cap.get(1) {
            let expression = capture.as_str();
            if let Some(value) = array.get(counter) {
                int_hash.insert(expression.to_string(), *value);
            }
        }counter +=1;
    } 
    int_hash
}

//function to get input string and array input from js, creates resulting hash
#[wasm_bindgen]   
pub fn assign_and_create_param_hash_js(input_str: &str, array:Vec<f64>) ->Array {
    let my_array:Vec<f64> = Vec::from(array); 
    let list = RegExpEnum::HashParameterLeftOnly.check_reg_ex().unwrap();
    let my_hash: LinkedHashMap<String, f64> = assign_user_parameters_js(list, my_array ,input_str);  
    let map_array: Array = send_linked_hash_to_vecter_js(my_hash);
    map_array

}




// function that converts a hash to an array for Javascript to read
pub fn send_linked_hash_to_vecter_js<T: ToJsValue>(para_hash: LinkedHashMap<String, T>) -> Array {
    let js_array = Array::new();

    for (key, value) in para_hash.into_iter() {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("key"), &JsValue::from_str(&key)).unwrap();
        let js_value = value.to_js_value();
        Reflect::set(&obj, &JsValue::from_str("value"), &js_value).unwrap();
        js_array.push(&obj);
    }
    js_array
}
// module implemented for send_linked_hash_to_vecter_js
mod to_js_value {
    use super::*;

    pub trait ToJsValue {
        fn to_js_value(&self) -> JsValue;
    }

    impl ToJsValue for f64 {
        fn to_js_value(&self) -> JsValue {
            JsValue::from_f64(*self)
        }
    }

    impl ToJsValue for bool {
        fn to_js_value(&self) -> JsValue {
            JsValue::from_bool(*self)
        }
    }

    impl ToJsValue for LogicalOp {
        fn to_js_value(&self) -> JsValue {
            let logical_op_str = match self {
                LogicalOp::AND => "AND",
                LogicalOp::OR => "OR",
            };
            JsValue::from_str(logical_op_str)
        }
    }
}





// fn to get logical hash array from given input string
#[wasm_bindgen]
pub fn get_logical_hash_array_js(input_str: &str) -> Array {
    let logical_hash = create_logical_hash(input_str);
    send_linked_hash_to_vecter_js(logical_hash)
}

// fn to get bool hash array from given input string and input array of parameters values
#[wasm_bindgen]
pub fn get_bool_hash_array_js(input_str: &str, array: Vec<f64>) -> Array {
    let my_array: Vec<f64> = Vec::from(array);
    let my_hash = create_user_parameters_hash(my_array, input_str);
    let bool_hash = create_bool_hash(input_str, &my_hash);
    send_linked_hash_to_vecter_js(bool_hash)
}

// fn to get an array of the main result modularly and final result
#[wasm_bindgen]
pub fn result_function(input_str: &str, array: Vec<f64>)-> Array {
    let my_array: Vec<f64> = Vec::from(array);
    let my_hash = create_user_parameters_hash(my_array, input_str);
    let logical_hash = create_logical_hash(input_str);
    let bool_hash = create_bool_hash(input_str, &my_hash);
    //let (main_result, modules_result, expression_mod_bool_list, expression_logical_list)=
    let (main_result, modules_result, _, _)=
        compute_final_results::compute_main_results(&my_hash, &bool_hash, &logical_hash );

    let main_result_array = js_sys::Array::new();
    main_result_array.push(&JsValue::from_bool(main_result));

    let modules_result_array = js_sys::Array::new();
    for value in modules_result {
        modules_result_array.push(&JsValue::from_bool(value));
    }

    js_sys::Array::of2(&main_result_array, &modules_result_array)
}

// sub function to create user parameters
fn create_user_parameters_hash(array: Vec<f64>, input_str: &str) -> LinkedHashMap<String, f64> {
    let list = RegExpEnum::HashParameterLeftOnly.check_reg_ex().unwrap();
    assign_user_parameters_js(list, array, input_str)
}

// sub function to create logical hash
fn create_logical_hash(input_str: &str) -> LinkedHashMap<String, LogicalOp> {
    let regular_exp_factors = RegExpEnum::OutsideParenthesis.check_reg_ex().unwrap();
    let mut logical_hash: LinkedHashMap<String, LogicalOp> = LinkedHashMap::new();
    hash_string_logicalop::create_hash_for_logiclop_dictionary(
        &regular_exp_factors, input_str, &mut logical_hash
    );
    logical_hash
}

// sub function to create bool hash
fn create_bool_hash(input_str: &str, my_hash: &LinkedHashMap<String, f64>) -> LinkedHashMap<String, bool> {
    let regular_exp_parentesis = RegExpEnum::InsideParenthesis.check_reg_ex().unwrap();
    let mut bool_hash: LinkedHashMap<String, bool> = LinkedHashMap::new();
    hash_bool_parameter_evaluation::create_parameter_bool_dictionary(
        &regular_exp_parentesis, input_str, &mut bool_hash, my_hash
    );
    bool_hash
}


//nnnnnn
/* 
#[wasm_bindgen]
pub fn result_function1(input_str: &str, array: Vec<f64>)-> Array {
    let my_array: Vec<f64> = Vec::from(array);
    let my_hash = create_user_parameters_hash(my_array, input_str);
    let logical_hash = create_logical_hash(input_str);
    let bool_hash = create_bool_hash(input_str, &my_hash);
    //let (main_result, modules_result, expression_mod_bool_list, expression_logical_list)=
    let (main_result, modules_result, _, _)=
        compute_final_results::compute_main_results(&my_hash, &bool_hash, &logical_hash );

    let main_result_array = js_sys::Array::new();
    main_result_array.push(&JsValue::from_bool(main_result));

    let modules_result_array = js_sys::Array::new();
    for value in modules_result {
        modules_result_array.push(&JsValue::from_bool(value));
    }

    let result_array = js_sys::Array::new();
    result_array.push(&JsValue::from(main_result_array));
    result_array.push(&JsValue::from(modules_result_array));
    
    result_array
}
*/

//aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
/* 
#[wasm_bindgen] 
pub fn get_logical_hash_array_js(input_str: &str)-> Array {
    let regular_exp_factors = RegExpEnum::OutsideParenthesis.check_reg_ex().unwrap();
    let mut logical_hash: LinkedHashMap<String, LogicalOp> = LinkedHashMap::new();
    hash_string_logicalop::create_hash_for_logiclop_dictionary(
        &regular_exp_factors, input_str, &mut logical_hash);
    send_linked_hash_to_vecter_js(logical_hash)
}

#[wasm_bindgen] 
pub fn get_bool_hash_array_js(input_str: &str, array:Vec<f64>)->Array{
    let my_array:Vec<f64> = Vec::from(array); 
    let list = RegExpEnum::HashParameterLeftOnly.check_reg_ex().unwrap();
    let my_hash: LinkedHashMap<String, f64> = assign_user_parameters_js(list, my_array ,input_str);
    
    let regular_exp_parentesis = RegExpEnum::InsideParenthesis.check_reg_ex().unwrap();
    let mut bool_hash: LinkedHashMap<String, bool> = LinkedHashMap::new();
    hash_bool_parameter_evaluation::create_parameter_bool_dictionary(
        &regular_exp_parentesis, input_str, &mut bool_hash,&my_hash);
    send_linked_hash_to_vecter_js(bool_hash)
}

#[wasm_bindgen]
pub fn result_function(input_str: &str, array:Vec<f64>) {
    let my_array:Vec<f64> = Vec::from(array);
    let list = RegExpEnum::HashParameterLeftOnly.check_reg_ex().unwrap();
    let my_hash: LinkedHashMap<String, f64> = assign_user_parameters_js(list, my_array ,input_str);

    let regular_exp_factors = RegExpEnum::OutsideParenthesis.check_reg_ex().unwrap();
    let regular_exp_parentesis = RegExpEnum::InsideParenthesis.check_reg_ex().unwrap();
    let mut logical_hash: LinkedHashMap<String, LogicalOp> = LinkedHashMap::new();
    let mut bool_hash: LinkedHashMap<String, bool> = LinkedHashMap::new();
    // fn to assess the expressions for Logicical Operators-op and Conditional Operators-exp
    hash_string_logicalop::create_hash_for_logiclop_dictionary(
        &regular_exp_factors, input_str, &mut logical_hash);

    hash_bool_parameter_evaluation::create_parameter_bool_dictionary(
        &regular_exp_parentesis, input_str, &mut bool_hash,&my_hash);
}


*/



/* pub fn send_linked_hash_to_vecter_js(para_hash: LinkedHashMap<String, f64>) -> Array{
    
    let js_array = Array::new();

    for (key, value) in para_hash.into_iter() {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("key"), &JsValue::from_str(&key)).unwrap();
        Reflect::set(&obj, &JsValue::from_str("value"), &JsValue::from_f64(value)).unwrap();
        js_array.push(&obj);
    }
    js_array 
}

pub fn send_linked_hash_to_vecter_js_bool(para_hash: LinkedHashMap<String, bool>) -> Array {
    let js_array = Array::new();

    for (key, value) in para_hash.into_iter() {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("key"), &JsValue::from_str(&key)).unwrap();
        Reflect::set(&obj, &JsValue::from_str("value"), &JsValue::from_bool(value)).unwrap();
        js_array.push(&obj);
    }

    js_array
}

*/

/*     
    let regular_exp_factors = RegExpEnum::OutsideParenthesis.check_reg_ex().unwrap();
    let regular_exp_parentesis = RegExpEnum::InsideParenthesis.check_reg_ex().unwrap();
    let mut logical_hash: LinkedHashMap<String, LogicalOp> = LinkedHashMap::new();
    let mut bool_hash: LinkedHashMap<String, bool> = LinkedHashMap::new();
    // fn to assess the expressions for Logicical Operators-op and Conditional Operators-exp
    hash_string_logicalop::create_hash_for_logiclop_dictionary(
        &regular_exp_factors, expression_str, &mut logical_hash);

    hash_bool_parameter_evaluation::create_parameter_bool_dictionary(
        &regular_exp_parentesis, expression_str, &mut bool_hash,&parameter_hash);

 
 */









 /* 
 ////////////////
 /// 
 ///
 
#[derive(Debug,Serialize)]
pub struct Entry {
    key: String,
    value: f64,
}

//creates a hash and assigns input array to variables from parameter_input for js
pub fn assign_user_parameters_js(list: Regex, array: Vec<f64>, expx: &str) -> LinkedHashMap<String, f64> {
    let mut int_hash: LinkedHashMap<String, f64> = LinkedHashMap::new();
    let mut counter: usize = 0;
    for cap in list.captures_iter(expx) {
        if let Some(capture) = cap.get(1) {
            let expression = capture.as_str();
            if let Some(value) = array.get(counter) {
                int_hash.insert(expression.to_string(), *value);
            }
        }counter +=1;
    } 
    int_hash
}

//function to get input string and array input from js, creates resulting hash
#[wasm_bindgen]   
pub fn assign_and_create_param_hash_js(input_str: &str, array:Vec<f64>) ->Array {

    let my_array:Vec<f64> = Vec::from(array); 
    let list = RegExpEnum::HashParameterLeftOnly.check_reg_ex().unwrap();
    let my_hash: LinkedHashMap<String, f64> = assign_user_parameters_js(list, my_array ,input_str);  

    let map_hash: Array = send_linked_hash_to_vecter_js(my_hash);
    map_hash
/* 
    let json_string = serde_json::to_string(&map_hash).expect("Serialization failed");
    JsValue::from_str(&json_string) */

}

  */