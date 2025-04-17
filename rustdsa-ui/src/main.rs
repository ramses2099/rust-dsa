#![allow(unused_imports)]
use parenthesis_match::{ par_checker1, par_checker2, par_checker3 };

mod parenthesis_match;

fn main() {
    // let sa = "()(())";
    // let sb = "()((()";
    // let res1 = par_checker1(sa);
    // let res2 = par_checker1(sb);
    // println!("{sa} balanced:{res1}, {sb} balanced: {res2}");

    // let sa = "(){}[]";
    // let sb = "(){)[}";
    // let res1 = par_checker2(sa);
    // let res2 = par_checker2(sb);
    // println!("{sa} balanced:{res1}, {sb} balanced: {res2}");

    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    let res1 = par_checker3(sa);
    let res2 = par_checker3(sb);
    println!("{sa} balanced:{res1}, {sb} balanced: {res2}");
}
