fn main() {
    let  p = 520000;
    let  r = 10;
    let  t = 5;
    // simple intrest
    let  a = p * (1 + (r / 100)) * t;
    println!("a is :   {}" , a);
    let si = a - p;
    println!(" simple intrest is {}", si );

}
