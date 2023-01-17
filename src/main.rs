// Bring `HashMap` into scope
use std::collections::HashMap;

fn main() -> () {
    // `mut` to opt-in to mutation
    // `.skip()` to skip the first arg (the location of the package)
    let mut args = std::env::args().skip(1);

    // `.unwrap()` to crash the program if the value ain't found
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let db = Database::new();

    let t = db.ok().unwrap().to_string();
    let contents = format!("{}{}\t{}\n", t, key, value);

    std::fs::write("kv.db", contents).expect("Error writing to file");

    // Exclamation signifies the func is a macro
    println!("Key: {} and Value: {}", key, value);
}

struct Database {
    inner: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Result::Ok(content) => content,
        //     Result::Err(err) => return Err(err),
        // };

        // Same as above ^
        let contents = std::fs::read_to_string("kv.db")?;
        let mut inner = HashMap::new();

        for line in contents.lines() {
            let chunks: Vec<&str> = line.split("\t").collect();
            if chunks.len() != 2 {
                todo!("Return error")
            }

            let key = chunks[0];
            let value = chunks[1];

            inner.insert(key.to_owned(), value.to_owned());

            // Debugger
            // dbg!(chunks);
        };

        Ok(
            Database {
                inner
            }
        )
    }

    fn to_string(&self) -> String {
        let mut final_string = String::from("");

        for k in self.inner.iter() {
            final_string = format!("{}{}\t{}\n", final_string, k.0, k.1);
        }

        return final_string;
    }
}