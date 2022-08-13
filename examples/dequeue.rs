use collections_rust::Dequeue;

fn main() {
    let mut dequeue = Dequeue::new();

    dequeue.push_back(1);
    dequeue.push_back(0);
    dequeue.push_back(1);

    println!("{dequeue:?}")
}
