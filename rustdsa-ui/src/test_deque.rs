#![allow(unused_imports)]

use rustdsa_lib::deque::Deque;

pub fn basic() {
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1);
    let _r2 = d.add_front(2);
    let _r3 = d.add_front(3);
    let _r4 = d.add_front(4);

    if let Err(error) = d.add_front(5) {
        println!("add_front error: {error}");
    }
    println!("{:?}", d);

    match d.remove_rear() {
        Some(data) => println!("remove rear data {data}"),
        None => println!("deque is empty"),
    }
    //
    match d.remove_front() {
        Some(data) => println!("remove front data {data}"),
        None => println!("deque is empty"),
    }
    //
    println!("empty: {}, len: {},", d.is_empty(), d.len());
    println!("full: {}, {:?}", d.is_full(), d);
    //
    d.clear();
    println!("{:?}", d);
}

pub fn iter() {
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1);
    let _r2 = d.add_front(2);
    let _r3 = d.add_front(3);
    let _r4 = d.add_front(4);

    let sum1 = d.iter().sum::<i32>();
    let mut added = 0;
    for item in d.iter_mut() {
        *item += 1;
        added += 1;
    }
    let sum2 = d.iter().sum::<i32>();
    println!("{sum1} + {added} = {sum2}");
    assert_eq!(14, d.iter().sum::<i32>());
}
