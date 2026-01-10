fn main() {
    let x: i32 = -5;
    let y: u32 = 1;
    let z: f32 = 1000.00;

    println!("x:{}, y:{}, z:{}", x, y ,z);

    // let is_male = true;
    // let is_above_18 = true;

    // if is_male {
    //     println!("U r a male");
    // } else {
    //     println!("U r not a male");
    // }

    // if is_male && is_above_18 {
    //     println!("U r not a legal");
    // }

    // let greeting = String::from("hello world");
    // println!("{}", greeting);

    // let s = "Hello Rust";
    // println!("{}", s);

    // let minor: bool = false;
    // if minor{
    //     println!("Is minor")
    // }else{
    //     println!("Not minor")
    // }

    // for i in 0..11 {
    //     print!("{} ", i)
    // }

    let mut x = String::from("Probal");
    x.push_str(" Ghosh");
    println!("{}", x);

}
