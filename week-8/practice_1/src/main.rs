fn main() {
    //using vec::new();
    let v : Vec<i64>= Vec::new();

    //printing the size vector
    println!("\nThe lenght of the Vec::new is:{}",v.len());

    //using macro
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    //printing the size of vector
    println!("\nThe length of the vec macro is: {}",v.len());
}
