#[allow(unused)]

use js_sys;

use crate::util_dependencies::*;
//   use wasm_bindgen::prelude::*;
//use js_sys::{RegExp,Reflect};
//   use crate::regular_expression_list::RegExpEnum;
//

//   use serde::{Serialize, Deserialize};
//   use serde_json;

/* //#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JsTypeEx {
    pub field1: Field1
}

//#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Field1 {
    pub string: String,
    pub i8: i8
}
 */

// https://docs.rs/serde-wasm-bindgen/latest/serde_wasm_bindgen/

// fn to check if parameters are integer of type i8
//#[wasm_bindgen]
pub fn check_if_int(expression:&str) -> i8 {
    loop {
        println!("Enter value of variable: {}",expression);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<i8>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid i8 value."),
        }
    }
}

// fn to enter parameters of user input variables


//pub fn assign_user_parameters_hash(list:Regex, expx:&str) ->LinkedHashMap<String, i8> {
//#[wasm_bindgen( typescript_type = "Regex")]
pub fn assign_user_parameters_hash(list:Regex, expx:&str) ->LinkedHashMap<String, i8> {
    
    let mut int_hash : LinkedHashMap<String, i8> = LinkedHashMap::new();
    for cap in list.captures_iter(expx) { 
        if let Some(capture) = cap.get(1) {
            let expression = capture.as_str();
            let input: i8 = check_if_int(expression);
            int_hash.insert(expression.to_string(), input);
        }
    }
    int_hash
}






/* 
pub fn assign_my_input_param(input_str: &str, array:Vec<i8>)-> (String, i8){
    let my_array:Vec<i8> = Vec::from(array);
    let list = RegExpEnum::HashParameterLeftOnly.check_reg_ex().unwrap();
    let my_hash: LinkedHashMap<String, i8> = assign_user_parameters_hash2(list, my_array ,input_str);
    if let Some((key, value)) = my_hash.front() {
        // Do something here with key and value
        let my_string = key.clone();
        let my_num = *value;

        (my_string, my_num)
    //int_hash
    }else {
        ("default".to_string(),0)
    }
    //my_hash
}



//#[wasm_bindgen]


#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
    let field1: (String, i8) = ("yo".to_string(), 3);
    let example_var = [field1.0, field1.1.to_string()];
    let json_str = serde_json::to_string(&example_var).unwrap();
    JsValue::from_str(&json_str)
}


#[wasm_bindgen]
pub struct ValueCollector {
    values: Vec<String>,
}


#[wasm_bindgen]
impl ValueCollector {
    pub fn new() -> ValueCollector {
        ValueCollector { values: Vec::new() }
    }

    pub fn add_value(&mut self, value: &str) {
        self.values.push(value.to_string());
    }

    pub fn get_values(&self) -> js_sys::Array {
        self.values.iter().map(|s| JsValue::from_str(s)).collect()
    }
}

*/

/* 
#[wasm_bindgen]
pub fn assign_user_parameters_hash2(pattern: &str, expx: &str) -> JsValue {
    let reg_exp = RegExp::new(pattern, r"[@#$%^&*]");//.unwrap();
    let mut int_hash : LinkedHashMap<String, i8> = LinkedHashMap::new();

    let global = JsValue::from_str(r"[@#$%^&*]");
    let captures = reg_exp.exec(expx);
    let captures_array = captures.unwrap();
    let capture = captures_array.get(1);

    for cap in 0..captures_array.length() {
        let capture = captures_array.get(cap);
        if let Some(expression) = capture.as_string() {
            let input: i8 = check_if_int(expression.as_str());
            int_hash.insert(expression, input);
        }
    }

    //JsValue::from_serde(&int_hash).unwrap()
    capture
}
 */


// to sub



