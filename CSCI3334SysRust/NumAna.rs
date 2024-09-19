fn main(){
    // Create an array of 10 integer numbers of your choice
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Implement a function is_even(n: i32) -> bool
    // This function returns true if a number is even, false otherwise
    fn is_even(n: i32) -> bool{
        n % 2 == 0
    }

    // Use a for loop to iterate through the array
    for &num in &numbers {
        // Check if the number is divisible by both 3 and 5
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        }
        // Check if the number is divisible by 3
        else if num % 3 == 0 {
            println!("{}: Fizz", num);
        }
        // Check if the number is divisible by 5
        else if num % 5 == 0 {
            println!("{}: Buzz", num);
        }
        // Check if number is even using the is_even function
        else if is_even(num) {
            println!("{}: Even", num);
        }
        // If none of the above conditions are met, it is odd
        else {
            println!("{}: Odd", num);
        }
    }

    // Use a while loop to find and print the sum of all numbers in the array
    let mut sum = 0;    // Variable to store the sum
    let mut index = 0;  // Index variable for the while loop
    while index < numbers.len() {
        sum += numbers[index];    // Add the current number to the sum
        index += 1;               // Move to the next index
    }
    println!("Sum of all numbers: {}", sum); // Print the sum

    // Use a loop to find and print the largetest number in the array
    let mut largest = numbers[0];   // Initialize with the first number
    for &num in &numbers[1..] {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest); //Print the largest number
}