fn main() {
    //while true
    let mut x = 0;
    loop {
        x += 1;
        println!("x={}",x);

        if x==15 {
            break;
        }
    }
}
fn main(){
    println!("Enter the number of candidates(max 50):");



let mut num_candidates_str = String::new();
io::stdin().read_line(&mut num_candidates_str).expect("Failed to read line");
let num_candidates: u32 = num_candidates_str.trim().parse().expect("Please enter a number");