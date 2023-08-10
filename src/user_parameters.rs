use crate::util_dependencies::*;

// fn to check if parameters are integer of type i8
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
