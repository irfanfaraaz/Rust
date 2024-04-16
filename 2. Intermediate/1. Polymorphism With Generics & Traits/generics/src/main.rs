struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand::new("open".to_owned(), "https://www.rust-lang.org".to_owned());

    let cmd2 = BrowserCommand::new("Zoom".to_owned(), 200);

    cmd1.print_payload();
    // cmd2.print_payload(); wont work here
    let p1 = cmd2.get_payload();
    println!("{}", p1);

    serialize_payload(p1);
}

fn serialize_payload<T>(payload: T) -> String {
    "placeholder".to_owned()
}
