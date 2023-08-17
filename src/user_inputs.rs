use crate::util_dependencies::{io, wasm_bindgen};

// for white spaces
#[wasm_bindgen]
pub fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|&c| !c.is_whitespace()).collect()
}

//checking for input
pub fn input_expression() -> String {
    loop {
        let mut input_string = String::new();

        println!("Enter String");
        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {
                let input = input_string.trim().to_string();
                let trimmed_input = remove_whitespace(input.as_str());
                return trimmed_input                                
            }
            Err(_) => {
                println!("Error: Invalid input. 
                \nExample of allowed Expression Format:
                \n(a==6)&&(b==7)||(c1>=6)&&(c2>8)||(d_1!=5)\nTry again");
                input_string.clear();
                continue;
            }
        }
    }
}


// fn to confirm the user response of expression
pub fn confirm_user_expression() -> bool {
    println!("Is this expression correct(t/f)?");
    let mut input_string = String::new();
    loop {
        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {
                let trimmed_str = input_string.trim();
                if trimmed_str == "t" || trimmed_str == "T" { return true; } 
                else if trimmed_str == "f" || trimmed_str == "F" { return false; }
                else {
                    println!("Enter either t or f");
                    input_string.clear(); 
                    continue;
                }
            }
            Err(_) => {
                println!("Error: Invalid input. Try again (t/f)");
                input_string.clear();
                continue;
            }
        }
    }
}

//fn to validate the response of the user for expression
pub fn check_expression(confirm: bool, expression: &str) -> String {
    if confirm {
        println!("You entered: \n{:?}", expression);
        expression.to_string()
    } else {
        println!("Re-enter expression");
        let new_expression = input_expression();
        check_expression(confirm_user_expression(), &new_expression)
    }
}
