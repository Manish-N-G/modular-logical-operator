#[allow(unused)]
use wasm_bindgen::prelude::*;
use crate::util_dependencies::*;
use crate::hash_string_logicalop;
use crate::hash_bool_parameter_evaluation;
use crate::user_inputs::remove_whitespace;
use crate::regular_expression_list::RegExpEnum;
use serde::Serialize; //Deserialize
use crate::LogicalOp;


//use serde_json;


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

}


use js_sys::{Array, Reflect};
use js_sys::Object;

pub fn send_linked_hash_to_vecter_js(para_hash: LinkedHashMap<String, f64>) -> Array{
    
    let js_array = Array::new();

    for (key, value) in para_hash.into_iter() {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("key"), &JsValue::from_str(&key)).unwrap();
        Reflect::set(&obj, &JsValue::from_str("value"), &JsValue::from_f64(value)).unwrap();
        js_array.push(&obj);
    }
    js_array 
}


/*  ////// backup
pub fn assign_my_input_param_js2(input_str: &str, array:Vec<i8>)->LinkedHashMap<String, bool>{
    let my_array:Vec<f64> = Vec::from(array); 
    let list = RegExpEnum::HashParameterLeftOnly.check_reg_ex().unwrap();
    let my_hash: LinkedHashMap<String, f64> = assign_user_parameters_js(list, my_array ,input_str);
    
    let regular_exp_factors = RegExpEnum::OutsideParenthesis.check_reg_ex().unwrap();
    let regular_exp_parentesis = RegExpEnum::InsideParenthesis.check_reg_ex().unwrap();
    let mut logical_hash: LinkedHashMap<String, LogicalOp> = LinkedHashMap::new();
    let mut bool_hash: LinkedHashMap<String, bool> = LinkedHashMap::new();
    
    hash_string_logicalop::create_hash_for_logiclop_dictionary(
        &regular_exp_factors, input_str, &mut logical_hash);

    hash_bool_parameter_evaluation::create_parameter_bool_dictionary(
        &regular_exp_parentesis, input_str, &mut bool_hash,&my_hash);
  
    
    my_hash;
    bool_hash

}
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

/* pub fn send_linked_hash_to_vecter_js(my_hash:LinkedHashMap<String, f64>) -> Vec<Entry> {

    let hash_entries: Vec<Entry> = my_hash
        .into_iter()
        .map(|(key, value)| Entry { key, value })
        .collect();

    hash_entries
} 
 */
use js_sys::{Array, Reflect};
use js_sys::Object;

pub fn send_linked_hash_to_vecter_js(para_hash: LinkedHashMap<String, f64>) -> Array{
    
    let js_array = Array::new();
    let mut para_hash: LinkedHashMap<String, f64> =LinkedHashMap::new();

    for (key, value) in para_hash.into_iter() {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("key"), &JsValue::from_str(&key)).unwrap();
        Reflect::set(&obj, &JsValue::from_str("value"), &JsValue::from_f64(value)).unwrap();
        js_array.push(&obj);
    }
    js_array 
}

#[wasm_bindgen]
pub fn test_new() -> Array{
    
    let js_array = Array::new();
    let mut para_hash: LinkedHashMap<String, f64> =LinkedHashMap::new();
    // where we assign eg a1a1_a1=6 ,b=2, c_222=7, ddd=3
    para_hash.insert("a1a1_a1".to_string(), 6.0);
    para_hash.insert("b".to_string(), 2.0);
    para_hash.insert("c_222".to_string(), 7.0);
    para_hash.insert("ddd".to_string(), 3.0);

    for (key, value) in para_hash.into_iter() {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("key"), &JsValue::from_str(&key)).unwrap();
        Reflect::set(&obj, &JsValue::from_str("value"), &JsValue::from_f64(value)).unwrap();
        js_array.push(&obj);
    }
/*     for (s, v) in entry.stg.iter().zip(&entry.val) {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("value"), &JsValue::from_f64(*v)).unwrap();
        js_array.push(&obj);
    } */

    js_array 

}



  */