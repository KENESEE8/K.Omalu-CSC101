use std::io;

fn main() {
    println!(" Welcome to the Hair Dryer Carbon Footprint Calculator!");
    println!("Produced by your very own Ogochukwu , Kenechukwu and Jeffo ");

    // Collect user data for each category related to the use of a hair dryer
    let energy_emissions = calculate_energy_emissions();
    let hair_dryer_emissions = calculate_hair_dryer();

 // Sum up total emissions
    let total_emissions= energy_emissions + hair_dryer_emissions;
    
   println !("\nYour estimated annual carbon footprint from energy emission is: {}  CO₂e ", energy_emissions);

println !("\nYour estimated annual carbon footprint from hair dryer use is: {} CO₂e ", hair_dryer_emissions);

println !("\nYour estimated annual carbon footprint from energy emission and hair dryer use is: {} CO₂e",total_emissions );
}


fn calculate_energy_emissions() -> f64 {
    // Function to calculate emissions from energy use
    println!("\nEnergy Use (in kWh):");
    let electricity = get_input("Electricity use");
    let emission_factor = 0.233; // Average CO2 emissions per kWh

    electricity * emission_factor
}
fn calculate_hair_dryer() -> f64 {
    // Function to calculate emissions from hair dryer use
    println!("\nHair Dryer Use:");
    let hair_dryer_hours = get_input("How many hours do you spend using your hair dryer per week?");
    let hair_dryer_power = get_input("how much power does your hair dryer make use of? (watts)");
    let emission_factor = 0.233; // Average CO2 emissions per kWh

    (hair_dryer_hours * hair_dryer_power / 1000.0) * emission_factor * 52.0 // Multiply by 52 to get annual emissions
}

fn get_input(prompt: &str) -> f64 {
    // Function to get user input
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
