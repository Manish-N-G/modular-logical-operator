import init, {
    result_function_js,
    get_logical_hash_array_js,
    get_bool_hash_array_js,
    assign_and_create_param_array_js,
    enter_input_js,
    input_expression_client_js,
    count_my_input_param_js,
    remove_whitespace
  } from './pkg/logical_module_program.js';
  
  init().then(() => {

    const startButton = document.getElementById("startButton");
    const nextButton = document.getElementById("nextButton");
    //const finishedText = document.getElementById("finished");

    startButton.addEventListener("click", startLoop);
    nextButton.addEventListener("click", getValues);

    let input_str = "";
    let my_input = "";
    let currentLoop = 0;
    let my_num = 0;
    let para_array_values = [];
    
    function startLoop() {
        currentLoop = 0;
        para_array_values = [];
        input_str = document.getElementById("StringField").value;
        finished.style.display = input_testing.style.display = "none";

        //(a1a1_a1==6)&&(b_==7)||(c_222>=6)&&(ddd>88)||(ddd>88)
        if (input_expression_client_js(input_str)) {
            resultArea.textContent = `You Entered: `;
            span_input.textContent = ` ${input_str}`;
            my_input = enter_input_js(remove_whitespace(input_str));
            my_num = count_my_input_param_js(my_input);
            
            createInputField();
            output.style.display = "block";
            input_testing.innerHTML = "";
        } else {
            resultArea.textContent = `You Entered: `;
            span_input.textContent = ` Empty String, enter again`;
            document.getElementById("input_testing").innerHTML = "";

            const startloop_elements = [input, nextButton, finished, ResultLabel, resultError];
            startloop_elements.forEach(element => element.style.display = "none");
        }
    }
    
    function createInputField() {
        
        if (currentLoop < my_num) {
            var output = document.getElementById("output");
            output.innerHTML = "";

            document.getElementById("StringField").style.display = "inline";
            document.getElementById("nextButton").style.display = "block";

            const create_elements = [ResultLabel, var_param_id_res, logicalop_arr_id_res, bool_param_arr_id_res, resultError, span_error];
            create_elements.forEach(element => element.style.display = "none");

            var NumField = document.createElement("input");
            NumField.type = "number";
            NumField.placeholder = "Enter a variable value"+ (currentLoop + 1);
            NumField.id = 'input';
            output.appendChild(NumField);

        } else {
            nextButton.style.display = finished.style.display = "none";
            output.innerHTML = "";
        }
    }
    
    function getValues() {
        document.getElementById("input_testing").style.display = "inline";
        
        var inputValue = document.getElementById("input").value;
        if (inputValue.trim() === "") {
            return;
        }
        para_array_values.push(inputValue);
        
        currentLoop++;
        document.getElementById("input_testing").innerHTML ="Array Value Entered: "+" ["+para_array_values+"], "+" Values Inputted: " + currentLoop;
        if (currentLoop < my_num) {
            createInputField();
        }else {
            nextButton.style.display = input.style.display = "none";
            document.getElementById("finished").style.display = "block"; // Show Finished sign
            console.log("Values entered:", para_array_values);
            console.log("Checking for my_input",my_input);
            get_result();

        }
    }

    function get_result(){
        console.log("for input+array",my_input, para_array_values);

        const var_param = assign_and_create_param_array_js(my_input, para_array_values);
        const logicalop_arr = get_logical_hash_array_js(my_input);
        const bool_param_arr = get_bool_hash_array_js(my_input, para_array_values);

        console.log('Result var_param ===', var_param);
        console.log('Result logicalop_arr===', logicalop_arr);
        console.log('Result bool_param_arr===', bool_param_arr.length);

        if (var_param.length == para_array_values.length && var_param.length == my_num
        && var_param.length == logicalop_arr.length+1 && var_param.length == bool_param_arr.length){
            console.log('Result ===', var_param);

            displayArrayResults(var_param, 'var_param_id_res', 'Your Variable parameters:');
            displayArrayResults(logicalop_arr, 'logicalop_arr_id_res', 'Your Logical Array:');
            displayArrayResults(bool_param_arr, 'bool_param_arr_id_res', 'Your bool parameter array:');

            function displayArrayResults(array, id, label) {
                const div = document.getElementById(id);
                div.innerHTML = '';
                const labelDiv = document.createElement("div");
                labelDiv.className = "label";
                labelDiv.textContent = label;
                div.appendChild(labelDiv);

                array.forEach(obj => {
                    const formattedText = `Key: ${obj.key},   Value: ${obj.value}`;
                    const para = document.createElement("p"); // Create a new paragraph element
                    para.textContent = formattedText; // Set the content of the paragraph
                    div.appendChild(para); // Append the paragraph to the div
                });
            }
            document.getElementById("ResultLabel").style.display = "";
            const get_result_elements = [var_param_id_res, logicalop_arr_id_res, bool_param_arr_id_res];
            get_result_elements.forEach(element => element.style.display = "inline-block");
            get_module_arrays(my_input, para_array_values, var_param, logicalop_arr, bool_param_arr);
        
        }else {
            resultError.textContent = `Error: `;
            span_error.textContent = ` Incorrect String Syntax, check string`;
            resultError.style.display = span_error.style.display = "";

        }
    }

    
    function get_module_arrays(my_input, para_array_values, var_param, logicalop_arr, bool_param_arr){
        let mixed_result_array = result_function_js(my_input, para_array_values);
        console.log("Final reuslt ==",mixed_result_array);
        
        const outputDiv = document.getElementById('output_result');
        outputDiv.innerHTML = '';
        let previousOutput = ``;
        for (let i = 0; i < mixed_result_array[1].length - 1; i++) {
            const outputStr = `${i+1}. For [${i === 0 ? bool_param_arr[i].key : `Module Output(${i})`}]  Logical:${logicalop_arr[i].value} [${bool_param_arr[i + 1].key}] = Module Output(${i+1}): ${mixed_result_array[1][i]}`;
            const para = document.createElement('p');
            para.textContent = outputStr;
            outputDiv.appendChild(para);
            
            previousOutput = `Module Output(${i+1})`;
        }
        
        const lastOutputStr = `${mixed_result_array[1].length}. For [${previousOutput}]  Logical:${logicalop_arr[mixed_result_array[1].length - 2].value} [${bool_param_arr[mixed_result_array[1].length - 1].key}] = Module Output(${mixed_result_array[1].length}): ${mixed_result_array[1][mixed_result_array[1].length - 1]}`;
        const lastPara = document.createElement('p');
        lastPara.textContent = lastOutputStr;
        outputDiv.appendChild(lastPara);

        const finalDiv = document.getElementById('final_result');
        finalDiv.textContent = `${mixed_result_array[0][0]}`;
    
    }
    //(a1a1_a1==6)&&(b_==7)||(c_222>=6)&&(ddd>88)||(ddd>88)
    //(a1a1_a1==6)&&(b_==7)||(c_222>=6)

});