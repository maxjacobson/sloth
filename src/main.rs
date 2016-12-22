extern crate rustyline;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => println!("Line: {:?}", line),
            Err(err) => {
                println!("Error: {}", err);
                println!("Exiting..");
                break
            },
        }
    }
}
