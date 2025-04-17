# Rust Dsa
Data Structures &amp; Algorithms in Rust!

## ArrayList DSA

# Command for running integration tests
```
cargo test -p rustdsa-lib
```

# Command for test specific integration tests
```
test --test [filename for test]
```
# Simple Balanced Parentheses
```
#![allow(unused_imports)]
use parenthesis_match::{ par_checker1, par_checker2, par_checker3 };

mod parenthesis_match;

fn main() {
    let sa = "()(())";
    let sb = "()((()";
    let res1 = par_checker1(sa);
    let res2 = par_checker1(sb);
    println!("{sa} balanced:{res1}, {sb} balanced: {res2}");

    let sa = "(){}[]";
    let sb = "(){)[}";
    let res1 = par_checker2(sa);
    let res2 = par_checker2(sb);
    println!("{sa} balanced:{res1}, {sb} balanced: {res2}");

    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    let res1 = par_checker3(sa);
    let res2 = par_checker3(sb);
    println!("{sa} balanced:{res1}, {sb} balanced: {res2}");
}
```
# Converting Decimal Numbers to Binary Numbers
```
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
```