use collections_rust::Queue;

fn main() {
    let mut queue = Queue::new();

    queue.append(3);
    queue.append(5);
    queue.append(7);

    println!("{queue}");
}
