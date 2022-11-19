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
            temp_converter();
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


// extract a number and what the number nee
fn temp_converter()
{
    println!(" input temp as <value><F|C>");

    let mut temp_input = String::new();
    io::stdin()
        .read_line(&mut temp_input)
        .expect("failed to read the line");

    println!("got {}",temp_input);
    
   
    
    if temp_input.find('F') != None{
        // convert from F to c
        println!("F to C");
        let input_temp_val = extract_temp_val(&mut temp_input);

    }else if temp_input.find('C') != None {
        // convert from C to F
        println!("C to F");
        let input_temp_val = extract_temp_val(&mut temp_input);

    }else {
        println!("the string {} does not contain the units F | C ",temp_input);
    }
}

fn extract_temp_val(temp_str: &str) -> f64{
    let input_temp_val:f64= match temp_str.trim().parse() {
        Ok(num) => num,
        Err(_) => return f64::NAN,
    };
    return input_temp_val;
}