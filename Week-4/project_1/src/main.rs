// Rust program to calculate the the roots of a quadratic equation

use std::io;

fn main(){
    println!("Quadractic Equation Solver (ax^2 + bx + c = 0)");

    // Read a, b, and c
    let mut a = String::new();
    let mut b = String::new(); 
    let mut c = String::new();

    println!("Enter a: ");
    io::stdin().read_line(& mut a).expect("Not a valid string");
    let a:f64 = a.trim().parse().expect("Not a valid number");

    if a == 0.0 {
        println!("'a' cannot be zero in a quadratic equation");
        return;
    }

    println!("Enter b: ");
    io::stdin().read_line(& mut b).expect("Not a valid string");
    let b:f64 = b.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(& mut c).expect("Not a valid string");
    let c:f64 = c.trim().parse().expect("Not a valid number");

    // Discriminant, d = b^2 - 4ac
    let d = (b * b) - (4.0 * a * c);
    println!("\nDiscriminant, d = {}", d);

    // Solve for roots at all conditions. Root = (-b ± √d) / 2a
if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);

        println!("Discriminant is greater than zero: Two distinct roots.");
        println!("Root 1 = {:.2}", root1);
        println!("Root 2 = {:.2}", root2);
    }
else if d == 0.0 {
        let root = -b / (2.0 * a);

        println!("Discriminant is equal to zero: Exactly one real root.");
        println!("Root = {:.2}", root);
    }
else {
        println!("Discriminant is less than zero: No real roots.");
    }
}
