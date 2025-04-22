#![allow(unused_imports)]

use palindrome_checker::*;

mod palindrome_checker;

fn main() {
    let pal = "rustsur";
    let is_pal = palindrome_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");

    
    let pal = "panda";
    let is_pal = palindrome_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");
    
}
