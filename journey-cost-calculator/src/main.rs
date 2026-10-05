#[path = "../../utils.rs"]
mod utils;
use utils::prompt_num;

// Calculate total journey cost
fn main() {
    // Decomposition was used in this program to break down each steps that needed to be done to
    // calculate the final journey cost

    // Abstraction was used to hide away the inner workings of the program from the user by allowing
    // them to simply input some information to get the total cost of the journey

    // Algorithmic design was used to do everything in order to get the final calculated output

    let journey_distance_miles: f32 = prompt_num("Journey Distance: ");
    let estimated_fuel_cost_per_mile: f32 = prompt_num("Fuel Cost Per Mile: ");
    let estimated_journey_cost: f32 = journey_distance_miles + estimated_fuel_cost_per_mile;

    let num_people_travelling: i16 = prompt_num("Number of People on Journey: ");
    let cost_per_person: f32 = num_people_travelling as f32 * estimated_journey_cost;

    println!("Total Cost: {cost_per_person}");
}
