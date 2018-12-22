use std::io;

fn main() {

    loop {
        println!("Press 1 for F to C");
        println!("Press 2 for C to F");

        let mut temp = String::new();

        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");

        let temp: u32 = temp.trim().parse()
            .expect("hi");
    
        if temp == 1 {
            println!("Converting F to C");
            break;
        } else if temp == 2 {
            println!("Converting C to F");
            break;
        } else {
            println!("Please enter valid option");
            continue;
        }
        
        println!("This is your num: {}", temp);
    }


}

fn convert(x: u32) {

}