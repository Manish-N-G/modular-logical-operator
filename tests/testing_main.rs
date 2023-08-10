#![allow(unused)]
use logical_module_program::main_logic_function;



//#[cfg(test)] // manual testing
mod test_main{
    use super::*;

    //#[test] //manual testing
    fn main_function_test() {
        println!("Test of main function");
        main_logic_function();
        // test fo different conditions
        // eg - (a1a1_a1==6)&&(b_==7)||(c_222>=6)&&(ddd>88)||(e!=5)||(f<=2)
        // (a1==6)&&(b1==7)||(c1>=6)&&(d1>88)||(e1!=5)
        // (a==6)&&(b==7)||(c>=6)&&(d>88)||(e!=5)
        // ( v_1= =6) && (v_ 2 ==7 )|| ( v _3 >= 6)& &( v_4>8 8)  | |  (v _5 !  = 5)
        // Error: if (a1==6)&&(b1==7)||(c1>=6)&&(d1>88)||(e1!=5  --> incomplete parameters
        // Error: if (a1==6)&&(b1==7)(c1>=6)&&(d1>88)(e1!=5)    --> missing || or &&
        // Error: if (a1==6)&&(a1==7)(c1>=6)&&(d1>88)(e1!=5)    --> repeated variable names
    }    
}

