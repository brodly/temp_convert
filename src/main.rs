use std::io;

fn main() {
    // establish string variable 
        // "F" for C to F
        // "C" for F to C

    // check for valid input: 1 or 2
        // 1 sets string variable to "F"
        // 2 sets string variable to "C"

    // create function that takes in i32
        // if string variable is "F"
            // convert i32 to F
        // if string variable is "C"
            // convert i32 to "C"
    

    loop {
        println!("Press 1 for C to F");
        println!("Press 2 for F to C");

        let mut temp = String::new();

        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");

        let mut temp: u32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        temp = check(temp);
        
        println!("Converted temp: {}", temp);
    }
}

fn convert(temp: u32, convert_to: u32) -> u32 {
    if convert_to == 0 {
        println!("{}", temp);
        1
    }

    if convert_to == 1 {
        println!("{}", temp);
        2
    }
}

fn check(temp: u32) -> u32 {
    if temp == 1 {
            println!("Converting F to C");
            convert(temp, 0)
        } else if temp == 2 {
            println!("Converting C to F");
            convert(temp, 1)
        } else {
            println!("Please enter valid option");
            0
        }
}