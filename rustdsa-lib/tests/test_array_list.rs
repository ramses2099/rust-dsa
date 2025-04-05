#[test]
fn create_arraylist() {
    use rustdsa_lib::array_list::ArrayList;

    let arr: ArrayList<i32> = ArrayList::new();
    assert_eq!(arr.length(), 0);
}

#[test]
fn add_element_to_array_list(){
    use rustdsa_lib::array_list::ArrayList;
    let mut arr: ArrayList<i32> = ArrayList::new();
    arr.push(1);
    assert_eq!(arr.get(0), Some(&1));
    assert_ne!(arr.length(), 0);
}

#[test]
fn array_list_is_empty() {
    use rustdsa_lib::array_list::ArrayList;
    let arr: ArrayList<i32> = ArrayList::new();
    assert!(arr.is_empty());
}
#[test]
fn array_list_get_all_values() {
    use rustdsa_lib::array_list::ArrayList;
    let mut arr: ArrayList<i32> = ArrayList::new();
    arr.push(1);
    arr.push(2);
    arr.push(3);
    let arr2 = arr.get_all();
    let arr3 = vec![1,2,3];
    assert_eq!(arr2, &arr3);
}

#[test]
fn array_list_remove_element() {
    use rustdsa_lib::array_list::ArrayList;
    let mut arr: ArrayList<i32> = ArrayList::new();
    arr.push(1);
    arr.push(2);
    arr.push(3);
    arr.remove(1);
    let arr2 = arr.get_all();
    let arr3 = vec![1,3];
    assert_eq!(arr2, &arr3);
}