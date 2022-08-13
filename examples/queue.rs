use linked_list::Queue;

fn main() {
    // Seg fault example
    // let p: *mut i32 = std::ptr::null_mut();
    // unsafe {
    //     *p = 6;
    // }
    // println!("yay");

    let mut queue = Queue::new();

    queue.append(3);
    queue.append(5);
    queue.append(7);

    println!("{queue}");
}
