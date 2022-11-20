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
            fib_handler();
        }else if menu_input == 2 {
            // 12 days of christmas
            twelve_days();
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
    println!("input temp as <value><F|C>");

    let mut temp_input = String::new();
    io::stdin()
        .read_line(&mut temp_input)
        .expect("failed to read the line");

    println!("got {}",temp_input);
    // remove the chars
    let extra:String = temp_input.chars().filter(|c| c.is_digit(10)).collect();
    
    let input_temp_val: i32 = match extra.trim().parse() {
        Ok(num) => num,
        Err(_) => i32::MAX,
    };

    if temp_input.find('F') != None{
        // convert from F to c
        println!("F to C");
        let out_temp:i32 = convert_f_to_c(input_temp_val);
        println!("{out_temp}C");

    }else if temp_input.find('C') != None {
        // convert from C to F
        println!("C to F");
        let out_temp = convert_c_to_f(input_temp_val);
        println!("{out_temp}F");

    }else {
        println!("the string {} does not contain the units F | C ",temp_input);
    }
}


fn convert_f_to_c(f :i32) -> i32{
    (f - 32 ) * (9/5) 
}

fn convert_c_to_f(c: i32) ->  i32{
    (c * 9/5) +32
}

fn fib_handler(){
    println!("input number for fibonacci sequence");
    let mut fib_input = String::new();
    io::stdin()
        .read_line(&mut fib_input)
        .expect("failed to read the line");
    
    let fib_input:u32 = match fib_input.trim().parse() {
        Ok(num) => num,
        Err(_) => u32::MIN,
    };
    println!("fibonacci seq for {fib_input}");
    for n in (0..fib_input).rev() {
        let numb = fibonacci(n);
        println!("{numb}");
    }
}

fn fibonacci(n : u32) -> u32{
    if n <= 1 {
        1
    }else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn twelve_days() {
    let mut day = 0;
    let gifts = [
        "100m sandback sprint",
        "kettlebell swings",
        "box jumps",
        "deadlifts",
        "hangcleans",
        "front squats",
        "pushpress",
        "burpees",
        "wallballs",
        "pullups",
        "toes to bar",
        "handstand pushups"
        ];
    loop{
        if day != 0 {
            if day == 13 {
                break;
            } 
            println!("on the {} of christmas my Jeff made me do:",day);
    
            for n in (0..day).rev(){
                println!("{} x {}",n+1,gifts[n])
            }
        }
        day +=1;

    }
}