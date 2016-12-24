extern crate rustyline;
mod instapaper;
mod sloth;
mod config;
use sloth::SlothApp;

fn main() {
    let sloth = match SlothApp::new() {
        Ok(app) => app,
        Err(err) => {
            println!("Could not start app.");
            println!("{}", err);
            return;
        }
    };

    let mut rl = rustyline::Editor::<()>::new();

    loop {
        sloth.print_status();

        match rl.readline("> ") {
            Ok(line) => {
                if sloth.receive_input(line) {
                    break;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                println!("Exiting..");
                break;
            }
        }
    }
}
