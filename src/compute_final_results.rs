use crate::util_dependencies::LinkedHashMap;
use crate::evaluate_bool_result;
use crate::LogicalOp;


// fn to compute final results
pub fn compute_main_results(parameter_hash:&LinkedHashMap<String, i8>,
    bool_hash:&LinkedHashMap<String,bool>,
    logical_hash:&LinkedHashMap<String,LogicalOp>) -> 
    (bool, Vec<bool>, Vec<bool>, Vec<LogicalOp>){
   
   if parameter_hash.len() == bool_hash.len() 
       && parameter_hash.len() == (logical_hash.len()+1) {
       
       if let (Some(final_result),final_module,
       module_exp,module_op) = 
       evaluate_bool_result::compute_boolean_result(
       &bool_hash, &logical_hash) {

           (final_result, final_module, module_exp, module_op)
       } else {
           panic!("Error: ---Insufficient values in hash dictionaries.---");
       }
   } else {
       panic!("Error: ---- String type error------");
   }
}