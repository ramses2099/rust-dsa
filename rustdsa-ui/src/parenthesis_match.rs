use rustdsa_lib::stack::Stack;
// par_checker

pub fn par_checker1(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;
            } else {
                _ = stack.pop();
            }
        }
        index += 1;
    }

    balance && stack.is_empty()
}

// par_checker2

// check if parentheses match of varius symbols
fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

pub fn par_checker2(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        // check 3 open symbols simultaneous
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c){
                    balance = false;
                }
            }
        }
        index += 1;
    }

    balance && stack.is_empty()
}

// par_checker3
pub fn par_checker3(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        // open mark, push
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        } 
        // close mark, determine if balanced    
        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c){
                    balance = false;
                }
            }
        }
        index += 1;
    }

    balance && stack.is_empty()
}
