#![allow(unused_imports)]

use decimal_to_binary::{base_converter, divide_by_two};

mod decimal_to_binary;

fn main() {
    let num1 = 10;
    let num2 = 43;
    // let bin_str = divide_by_two(num1);
    // println!("{num1} = b{bin_str}");
    let bin_str = base_converter(num1,2);
    println!("{num1} = b{bin_str}");
    let hex_str = base_converter(num2,16);
    println!("{num2} = x{hex_str}");
    
}
