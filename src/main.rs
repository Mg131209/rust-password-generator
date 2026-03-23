//importing iputa and aoutput crate
use std::io;
use rand::Rng;
fn main() {
    println!("Password Generator");
   
    let length = loop {
        println!("Select a length for the pasword:");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Faild to read input");
        
        match input.trim().parse::<usize>(){
            Ok(num) if num > 0 => break num,
            Ok(_) => println!("Passowrd length must be greater than 0"),
            Err(_)=> println!("Plesa enter a valid number"),
        }
    };
    generate_password(length);
}

fn generate_password(length: usize){
    println!("Generating password of length {}", length);
    let mut password = String::from("");
    let mut i = 0;
    while i < length{
        password.push(random_ascii_char());
        i = i +1; 
    }
    println!("Your password got generated: {}", password);
}


// Generates a random ASCII character from printable characters (32-126)
fn random_ascii_char() -> char {
    let mut rng = rand::thread_rng();
    // ASCII printable characters range from 32 (space) to 126 (~)
    let ascii_code = rng.gen_range(32..=126);
    // Convert the ASCII code to a char
    ascii_code as u8 as char
}
