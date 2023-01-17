// Bring `HashMap` into scope
use std::collections::HashMap;

fn main() {
    // `mut` to opt-in to mutation
    // `.skip()` to skip the first arg (the location of the package)
    let mut args = std::env::args().skip(1);

    // `.unwrap()` to crash the program if the value ain't found
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let db = Database::new();

    let contents = format!("{}\t{}\n", key, value);

    std::fs::write("kv.db", contents);

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

        for line in contents.lines() {
            let chunks: Vec<&str> = line.split("\t").collect();

            // Debugger
            dbg!(chunks);
        };
        Ok(
            Database {
                inner: std::collections::HashMap::new()
            }
        )
    }
}