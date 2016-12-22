pub struct SlothApp {
}

impl SlothApp {
    pub fn new() -> SlothApp {
        SlothApp {}
    }

    pub fn print_status(&self) {
        println!("Not much going on yet!");
    }

    pub fn receive_input(&self, input: String) {
        if input == String::from("help") {
            self.print_help();
        } else {
            println!("Unrecognized input");
        }
    }

    fn print_help(&self) {
        println!("Help text will go here...");
    }
}
