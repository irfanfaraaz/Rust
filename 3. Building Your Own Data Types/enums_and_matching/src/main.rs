enum Command{
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32,i32),
    Replace{
        from: String,
        to: String,
    }
}

impl Command{
    fn serialize(&self) -> String {
        match self {
            Command::Undo => String::from(
                "{ \"cmd\": \"undo\" }"
            ),
            Command::Redo => String::from(
                "{ \"cmd\": \"redo\" }"
            ),
            Command::AddText(s) => {
                format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                )
            }
            Command::MoveCursor(line, column) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": {line}, \
                        \"column\": {column} \
                    }}"
                )
            }
            Command::Replace { from, to } => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\", \
                    }}"
                )
            }
        }
    }
}
fn main() {
    // let age = 35;
    // match age {
    //     1=> println!("Happy first birthday!"),
    //     13..=19 => println!("Happy teenage years!"),
    //     20..=39 => println!("You are in your 20s or 30s"),
    //     x => println!("You are {} years old", x),//x is catch all pattern
    // }
    
    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("Hello"));
    let cmd3 = Command::MoveCursor(3,5);
    let cmd4 = Command::Replace{
        from: String::from("Hello"),
        to: String::from("World"),
    };

    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());


}
