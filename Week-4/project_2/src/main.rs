// Rust program to calculate annual incentive

use std::io;

fn main() {
    println!("Employee Incentive Calculator");

    let mut exp = String::new();
    let mut age = String::new();

    // Read experience
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut exp).expect("Not a valid string");
    let is_experienced = exp.trim().to_lowercase() == "yes";

    // Read age
    println!("Enter age: ");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age: u8 = age.trim().parse().expect("Not a valid number");

    // Determine annual incentive
    let incentive: u32;

    if is_experienced && age >= 40 {
        incentive = 1_560_000;
    } else if is_experienced && age >= 30 {
        incentive = 1_480_000;
    } else if is_experienced && age < 28 {
        incentive = 1_300_000;
    } else if !is_experienced {
        incentive = 100_000;
    } else {
        incentive = 0;
    }

    println!("\nThe annual incentive is: N{}", incentive);
}