fn main() {
    let p: *mut i32 = std::ptr::null_mut();
    unsafe {
        *p = 6;
    }
    println!("yay");
}
