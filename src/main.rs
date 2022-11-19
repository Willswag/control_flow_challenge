// create a program to convert temperature units
// create a program to calc a fibonacci number
// create a program to print the 12 days of chrismas

use std::io;


fn main() {
    'menu_loop: loop
    {
        print_menu();

    }




}

fn print_menu(){
    let options = ["temp conversion", "fibonacci", "12 days of christmas", "exit"];
    let mut count = 0;
    println!("===== Control flow challenge ====");
    println!();
    for opt in options{
        println!("\t input {count} for {opt}");
        count += 1;
    }
    println!("");
}
