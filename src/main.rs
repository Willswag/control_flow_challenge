// create a program to convert temperature units
// create a program to calc a fibonacci number
// create a program to print the 12 days of chrismas

use std::io;


fn main() {
    'menu_loop: loop
    {
        let mut menu_input = String::new();
        print_menu();

        io::stdin()
            .read_line(&mut menu_input)
            .expect("failed to read the line");
        

        let menu_input:u32 = match menu_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if menu_input == 0{
            // temp conversion
            println!("not implemented");
        }else if menu_input == 1 {
                        // fibonacci
                        println!("not implemented");
        }else if menu_input == 2 {
            // 12 days of christmas
            println!("not implemented");
        }else if  menu_input == 3{
            // exit
            break 'menu_loop;
        }else {
            println!("bad input");
        }
        
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
    println!();
}
