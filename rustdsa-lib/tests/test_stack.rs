#[test]
fn test_stack_create_stack() {
    use rustdsa_lib::stack::Stack;

    let stack: Stack<i32> = Stack::new();
    assert_eq!(stack.is_empty(), true);
}

#[test]
fn test_stack_add_items() {
    // use rustdsa_lib::stack::Stack;

    // let mut stack: Stack<i32> = Stack::new();
    // assert_eq!(stack.is_empty(), true);
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);
    // stack.push(4);

    // assert_eq!(stack.is_empty(), false);
    // assert_ne!(stack.len(), 0);
    
}
 