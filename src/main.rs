//Memory mangement
// fn main() {
//     println!("These is Stack and Heap");
//     update_string()
// }


// fn update_string(){
//     let mut s = String::from("Initial string");
//     println!("Before update: {}", s);
//     println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

//     s.push_str(" and some additional");
//     println!("After update: {}", s);
//     println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
// }


//owernship
// fn main(){
//     let s1 = String::from("Hi there");
//     let s2 = s1; // moved ownership
//     print!("{}",s1);
// }

fn main(){
    let my_string = String::from("HI THERE");
    takes_ownership(my_string);
    println!("{}", my_string); //moved ownership
}


fn takes_ownership(some_string: String){
    println!("{}", some_string);
}