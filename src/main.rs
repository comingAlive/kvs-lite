use std::collections::HashMap;
use std::env::args;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;

struct Database {
    inner: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, Error> {
        let contents = read_to_string("kvs.txt")?;
        let mut inner: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            let chunks: Vec<&str> = line.split(' ').collect();
            let key = chunks[0];
            let value = chunks[1];
            inner.insert(key.parse().unwrap(), value.parse().unwrap());
        }

        Ok(Database { inner })
    }
}

fn main() {
    if !Path::new("kvs.txt").exists() {
        let _result = File::create("kvs.txt");
    }

    let mut args = args().skip(1);
    let args_len = args.len();

    let db = Database::new();

    if args_len == 1 {
        let key = args.next().unwrap();
        let unwrapped_db = db.unwrap().inner.clone();

        if !unwrapped_db.contains_key(&*key) {
            println!("\nPair with the key {:?} doesn't exist in the DB.\n", key);
            return;
        }

        print!("\n{}\n", unwrapped_db.get(&*key).unwrap())
    } else if args_len == 2 {
        let key = args.next().unwrap();
        let value = args.next().unwrap();
        let contents = format!("{} {}", key, value);

        if db.unwrap().inner.contains_key(&*key) {
            println!("\nThis key is already registered in the DB. To override its value add -f or --force flag after a pair.\nExample: \"kvs key value -f\"\n");
            return;
        }

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("kvs.txt")
            .unwrap();
        match writeln!(file, "{}", contents) {
            Err(e) => {
                eprintln!("\nCouldn't write to file: {}\n", e);
            }
            Ok(_c) => {
                let result = contents.split(" ").collect::<Vec<&str>>().join(" ");

                println!("\n{:?} was successfully added to the DB\n", result)
            }
        }
    } else if args_len == 3 {
        let key = args.next().unwrap();
        let value = args.next().unwrap();
        let flag = args.next().unwrap();

        if flag != "-f" && flag != "--force" {
            println!("\nWrong flag \"{}\"\n", flag);
            return;
        }

        let mut db_inner = db.unwrap().inner;

        if db_inner.contains_key(&*key) {
            db_inner.remove(&*key);
            let mut _file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("kvs.txt");

            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("kvs.txt")
                .unwrap();

            let mut owned_string = "".to_owned();
            for line in &db_inner {
                owned_string.push_str(&*line.0);
                owned_string.push_str(" ");
                owned_string.push_str(&*line.1);
                owned_string.push_str("\n");
            }

            owned_string.push_str(&*key);
            owned_string.push_str(" ");
            owned_string.push_str(&*value);
            owned_string.push_str("\n");

            match write!(file, "{}", owned_string) {
                Err(e) => {
                    eprintln!("\nCouldn't write to file: {}\n", e);
                }
                Ok(_c) => {
                    println!(
                        "\n{:?} was successfully updated with the new value \"{}\"\n",
                        key, value
                    )
                }
            }
            return;
        }
    } else {
        println!("\nto get a value -> \"kvs key\"\nto set a pair -> \"kvs key value\"\n")
    }
}
