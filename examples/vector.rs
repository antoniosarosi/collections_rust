use collections_rust::Vector;

fn main() {
    let mut v = Vector::new();

    v.push(5);
    v.push(6);

    println!("V[0] = {}, v[1] = {}", v[0], v[1]);
}
