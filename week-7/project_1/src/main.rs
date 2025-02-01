use std::io;

fn main() {
    println!("Hello, world!");
}

fn Trapezium() {
    let mut hT = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();

    println!("What do you want the height of the Trapezium to be?");
    io::stdin()
        .read_line(&mut hT)
        .expect("Failed to read input");

    let heightvalue: f32 = hT
        .trim()
        .parse()
        .expect("Invalid input, ensure you typed numbers only");

    println!("What do you want the first base of the Trapezium to be?");
    io::stdin()
        .read_line(&mut base1)
        .expect("Failed to read input");

    let base1value: f32 = base1
        .trim()
        .parse()
        .expect("Invalid input, ensure you typed numbers only");

    println!("What do you want the second base of the Trapezium to be?");
    io::stdin()
        .read_line(&mut base2)
        .expect("Failed to read input");

    let base2value: f32 = base2
        .trim()
        .parse()
        .expect("Invalid input, ensure you typed numbers only");

    let formulaT = (heightvalue / 2.0) * (base1value + base2value);
    println!("The Area of this Trapezium is {}", formulaT);
}

fn Rhombus() {
    let mut diagonall = String::new();
    let mut diagonal2 = String::new();

    println!("What do you want the first diagonal of the Rhombus to be?");
    io::stdin()
        .read_line(&mut diagonall)
        .expect("Failed to read input");
    let diagonallvalue: f32 = diagonall
        .trim()
        .parse()
        .expect("Invalid input, ensure you typed numbers only");

    println!("What do you want the second diagonal of the Rhombus to be?");
    io::stdin()
        .read_line(&mut diagonal2)
        .expect("Failed to read input");

    let diagonal2value: f32 = diagonal2
        .trim()
        .parse()
        .expect("Invalid input, ensure you typed numbers only");

    let formular = 0.5 * diagonallvalue * diagonal2value;
    println!("The Area of this Rhombus is {}", formular);
}

fn Parallelogram() {
    let mut baseP = String::new();
    let mut altitude = String::new();

    println!("What do you want the base of the Parallelogram to be?");
    io::stdin()
        .read_line(&mut baseP)
        .expect("Failed to read input");
    let basePvalue: f32 = baseP
        .trim()
        .parse()
        .expect("Invalid input, ensure you typed numbers only");

    println!("What do you want the altitude of the Parallelogram to be?");
    io::stdin()
        .read_line(&mut altitude)
        .expect("Failed to read input");

    let altitudevalue: f32 = altitude
        .trim()
        .parse()
        .expect("Invalid input, ensure you typed numbers only");

    let formulaP = basePvalue * altitudevalue;
    println!("The Area of this Rhombus is {}", formulaP);
}

fn Cube() {}
