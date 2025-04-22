#![allow(unused_imports)]

use rustdsa_lib::deque::Deque;

pub fn palindrome_checker(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());
    let mut is_pal: bool = true;
    for c in pal.chars() {
        let _r = d.add_rear(c);
    }

    while d.len() > 1 && is_pal {
        let head = d.remove_front();
        let tail = d.remove_rear();
        if head != tail {
            is_pal = false;
        }
    }

    is_pal
}
