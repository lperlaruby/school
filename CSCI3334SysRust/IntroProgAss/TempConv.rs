// Declare a constant for the freezing point of water in Fahrenheit (32°F)
const FREEZ_POI_F: f64 = 32.0;

// Function to convert Fahrenheit to Celsius
// Formula: (F - 32) * 5/9
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZ_POI_F) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
// Formula: C * 9/5 + 32
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZ_POI_F
}

fn main() {
    // Declare a mutable variable with a temperature in Fahrenheit
    let mut temp_f: f64 = 32.0;

    // Convert it to Celsius using the function
    let temp_c = fahrenheit_to_celsius(temp_f);

    // Print Fahrenheit temp to Celsius conversion
    println!("{:.2}°F is {:.2}°C", temp_f, temp_c);

    // Loop to convert and print the next 5 integer temperatures in Fahrenheit
    for _ in 1..=5 {
        // Increment the Fahrenheit temperature by 1 degree
        temp_f += 1.0;

        // Convert the updated Fahrenheit temperature to Celsius
        let temp_c = fahrenheit_to_celsius(temp_f);

        // Print the Fahrenheit to Celsius conversion
        println!("{:.2}°F is {:.2}°C", temp_f, temp_c);

        // Also demonstrate conversion back to Fahrenheit (if needed)
        let temp_f_back = celsius_to_fahrenheit(temp_c);
        println!("{:.2}°C is {:.2}°F", temp_c, temp_f_back);
    }
}
