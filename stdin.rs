use std::io;

fn main() {
    let mut input = String::new();

    // reads from stdin
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("Input: {}", input);
            println!("{} bytes read", _n);
        }
        Err(error) => println!("Error: {}", error),
    }

}
