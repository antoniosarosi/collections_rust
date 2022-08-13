use linked_list::LinkedList;

fn main() {
    // Seg fault example
    // let p: *mut i32 = std::ptr::null_mut();
    // unsafe {
    //     *p = 6;
    // }
    // println!("yay");

    let mut list = LinkedList::new();

    list.append(3);
    list.append(5);
    list.append(7);

    println!("{list}");
}
