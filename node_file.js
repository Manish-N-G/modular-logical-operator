//const { Console } = require('console');
//const my_app = require('./pkg/logical_module_program.js');
const my_app = require('./pkg/logical_module_program.js');
const rl = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout
});

let my_input;
let my_num;
let jsArray = [];
let array_numbers = [];

function getInput() {
    rl.question('Enter String?\n', name => {
        const input_bool = my_app.input_expression_client_js(name);
        if (input_bool) {
            console.log(`You entered: \n${my_app.enter_input_js(my_app.remove_whitespace(name))}`);
            my_input = my_app.enter_input_js(my_app.remove_whitespace(name));
            my_num = my_app.count_my_input_param_js(name);

            createInputField();
        } else {
            console.log(`You entered: An empty String, Try again`);
            getInput();
        }
    });
}

function createInputField() {
    console.log(`You entered into create`);
    jsArray = my_app.send_var_array_names_js(my_input);

    function getInputForArray(index) {
        if (index >= jsArray.length) {
            console.log(`Input Variable array`, array_numbers);
            get_result();
            rl.close();
            return;
        }

        const element = jsArray[index];
        rl.question(`Enter value for ${element}:\n`, input_num => {
            const parsedInput = parseFloat(input_num);
            if (!isNaN(parsedInput)) { // Check if input is a number
                array_numbers.push(parsedInput);
                getInputForArray(index + 1);
            } else {
                console.log(`Invalid input. Please enter a number.`);
                getInputForArray(index); // Retry for the same element
            }
        });
    }

    getInputForArray(0);
}

function get_result() {
    const var_param = my_app.assign_and_create_param_array_js(my_input, array_numbers);
    const logicalop_arr = my_app.get_logical_hash_array_js(my_input);
    const bool_param_arr = my_app.get_bool_hash_array_js(my_input, array_numbers);

    console.log('\nInputted Variable Parameters ==\n', var_param,'\n');
    console.log('Inputted logicalop Parameters==', logicalop_arr,'\n');
    console.log('Inputted Bool Parameters==', bool_param_arr,'\n');

    if (var_param.length == array_numbers.length && var_param.length == my_num
    && var_param.length == logicalop_arr.length+1 && var_param.length == bool_param_arr.length){
        
        console.log('RESULT: String entered is correct');
        get_module_arrays(my_input, array_numbers, var_param, logicalop_arr, bool_param_arr);
        
    }else {
        console.log('ERROR: Incorrect String Syntax, check string');
    }
}



function get_module_arrays(my_input, array_numbers, var_param, logicalop_arr, bool_param_arr){
    console.log('\n---Computing Results---\n');
    let mixed_result_array = my_app.result_function_js(my_input, array_numbers);
    
    for (let i = 0; i < mixed_result_array[1].length - 1; i++) {
        console.log(`${i+1}. For [${i === 0 ? bool_param_arr[i].key : `Module Output(${i})`}]  Logical:${logicalop_arr[i].value} [${bool_param_arr[i + 1].key}] = Module Output(${i+1}): ${mixed_result_array[1][i]}`);
        previousOutput = `Module Output(${i+1})`;
    }
    
    console.log(`${mixed_result_array[1].length}. For [${previousOutput}]  Logical:${logicalop_arr[mixed_result_array[1].length - 2].value} [${bool_param_arr[mixed_result_array[1].length - 1].key}] = Module Output(${mixed_result_array[1].length}): ${mixed_result_array[1][mixed_result_array[1].length - 1]}`);
    console.log('\n---Computing Results---\n');
    console.log('Result of Expression: ',mixed_result_array[0][0],'\n');
}

getInput();

//example strings expression
//(a1a1_a1==6)&&(b_==7)||(c_222>  =6)&&(ddd>88)||(ddd>88)
//(a1a1_a1==6)&&(b_==7)||(c_222>=6)

