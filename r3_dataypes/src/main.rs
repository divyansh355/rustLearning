use std::io;
fn main(){
    
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;

    println!("Sum Value: {sum}");
    println!("Difference Value: {difference}");
    println!("Product Value: {product}");
    println!("Quotient Value: {quotient}");
    println!("Truncated Value: {truncated}");
    println!("Remainder Value: {remainder}");


    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("True Value : {t}");
    println!("False Value : {f}");


    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("C value : {c}");
    println!("Z value : {z}");
    println!("Cat value : {heart_eyed_cat}");


    let tup = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of y is: {_y}");


    let a = [4, 9, 3, 1, 7];
    let first = a[0];
    let second = a[1];
    println!("First Value : {first}");
    println!("Second Value : {second}");    


    let a = [11, 21, 54, 56, 51];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}