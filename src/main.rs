extern crate rustyline;
mod sloth;
use sloth::SlothApp;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut sloth = SlothApp::new();

    loop {
        sloth.print_status();

        match rl.readline("> ") {
            Ok(line) => {
                sloth.receive_input(line);
            },
            Err(err) => {
                println!("Error: {}", err);
                println!("Exiting..");
                break
            },
        }
    }
}
