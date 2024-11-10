//Rust program to read the height of a person
//and then print if person is tall, dwarrf, or average height person


use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your height(in centimetres):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid nummber");

    if height>= 150.0 && height<=170.0
    {
    println!("You are average heighted");
    }
    else if height >= 170.0 && height <= 220.0 
    {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are dwarf");
    }
    else
    {
        println!("You are of abnormal height");
    }
}
