use std::{io, u8};
use rot_manipulator::Ciphertext;

fn main() {

    let mut input_cipher = String::new();
    let mut input_rot = String::new();
    
    println!("-=-=-=-=-=-=-=-=-=-=-=ROTator=-=-=-=-=-=-=-=-=-=-=-");
    println!("First, enter your ciphertext:");
    
    io::stdin()
    .read_line(&mut input_cipher)
    .expect("Insert a valid ciphertext!");

    let mut ciphertext = Ciphertext::new(input_cipher);

    loop {
        println!("\nCURRENT CIPHERTEXT: [ROT {}] {}\n",ciphertext.rot, ciphertext.current_text);
        println!("Type 'a' to list all ROT possibilities, a exact number of rotations or\n'exit' to finish the finish the program");

        input_rot.clear();
        io::stdin()
        .read_line(&mut input_rot)
        .expect("Insert a valid option");

        input_rot = String::from(input_rot.trim());

        match input_rot.as_str() {
            "a" => {
                println!("Original ciphertext: {}", ciphertext.original_text);
                println!("Listing all ROT possibilities:");
                for _ in 1..26 {
                    ciphertext.rotate(1);
                    println!("  [ROT {}]: {}", ciphertext.rot, ciphertext.current_text)
                }
                ciphertext.rotate(1);
            }
            "exit" => {
                println!("Finishing...");
                break;
            }
            _ => {
                match input_rot.parse::<u8>() {
                    Ok(rot) => {
                        ciphertext.rotate(rot);
                    }
                    Err(_) => println!("Invalid input, try again!")
                }
            }
        }
    }
}