use std::io;
use serde_json::{Value, self, json};

fn main() -> io::Result<()> {

    let mut json_buffer = String::new();

    io::stdin().read_line(&mut json_buffer)?;

    let obj: Value = serde_json::from_str(&json_buffer.to_owned())?;

    println!("{}", serde_json::to_string_pretty(&obj).unwrap());

    Ok(())
}
