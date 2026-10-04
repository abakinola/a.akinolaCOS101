use std::io;

fn main() {
    // Display the menu
    println!("------------------ RESTAURANT MENU -------------------");
    println!("P -> Poundo Yam / Edinkaiko Soup : N3,200");
    println!("F -> Fried Rice & Chicken        : N3,000");
    println!("A -> Amala & Ewedu Soup          : N2,500");
    println!("E -> Eba & Egusi Soup            : N2,000");
    println!("W -> White Rice & Stew           : N2,500");
    println!("-----------------------------------------------------");

    // Input food choice
    println!("\nPlease enter your choice (P, F, A, E, W): ");
    let mut food_code = String::new();
    io::stdin().read_line(&mut food_code).expect("Failed to read line");
    let food_code = food_code.trim().to_uppercase();

    // Determine price based on food choice
    let price: f64 = if food_code == "P" {
        3200.0
    } else if food_code == "F" {
        3000.0
    } else if food_code == "A" {
        2500.0
    } else if food_code == "E" {
        2000.0
    } else if food_code == "W" {
        2500.0
    } else {
        println!("Invalid food choice entered.");
        return;
    };

    // Input quantity
    println!("Enter quantity: ");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read line");
    let quantity:f64 = quantity.trim().parse().expect("Not a valid number");

    // Calculate total charge
    let mut total = price * quantity;

    // Apply 5% discount if total exceeds N10,000
    if total > 10000.0 {
        println!("\nYou qualify for a 5% discount!");
        let discount = total * 0.05;
        println!("Discount amount: N{:.2}", discount);
        total = total - discount;
    }
    println!("Total amount payable: N{:.2}", total);
}