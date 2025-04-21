#![allow(unused_imports)]

use hot_potato::hot_potato;

mod hot_potato;

fn main() {
    let names = vec!["mon", "tom", "kew", "lisa", "marry", "bob"];
    let survivor = hot_potato(names, 8);
    println!("the survival person is {survivor}");
}
