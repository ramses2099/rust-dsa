use rustdsa_lib::stack::Stack;

fn basic() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    println!("Size: {}, {:?}", s.len(), s);
    println!("Pop {:?}, Size {}", s.pop(), s.len());
    println!("Empty: {}, {:?}", s.is_empty(), s);
    s.clear();
    println!("{:?}", s);
}
//
fn peek() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    println!("{:?}", s);
    let peek_mut = s.peek_mut();
    if let Some(top) = peek_mut {
        *top = 4;
    }
    println!("top {:?}", s.peek().unwrap());
    println!("{:?}", s);
}
//
fn iter() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    let sum1 = s.iter().sum::<i32>();
    let mut addend = 0;
    for item in s.iter_mut() {
        *item += 1;
        addend += 1;
    }
    let sum2 = s.iter().sum::<i32>();
    println!("{sum1} + {addend} = {sum2}");
    assert_eq!(9, s.into_iter().sum::<i32>());
}
fn main() {
    basic();
    peek();
    iter();
}
