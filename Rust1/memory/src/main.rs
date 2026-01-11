fn main() {
    println!("These is Stack and Heap");
    update_string()
}


fn update_string(){
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    s.push_str(" and some additional");
    println!("After update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
}