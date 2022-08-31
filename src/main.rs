use serde_json::{self, Value};
use std::io;
use unescape::unescape;

fn pretty_print_value(value: &Value) {
    pretty_print_value_w_prefix(value, "".to_owned());
}

fn pretty_print_value_w_prefix(value: &Value, prefix: String) {
    println!(
        "{} {}",
        prefix,
        serde_json::to_string_pretty(value).unwrap()
    );
}

fn log_key(key: &String, source: &Value) {
    match source.as_object() {
        None => return,
        Some(object) => object.iter().for_each(|i| {
            let curr_key = i.0;
            let value = i.1;

            if curr_key == key {
                pretty_print_value_w_prefix(value, format!("{curr_key} =>"));
            }

            log_key(key, value);
        }),
    }
}

fn main() -> io::Result<()> {
    let mut json_buffer = String::new();

    io::stdin().read_line(&mut json_buffer)?;

    json_buffer = unescape(&json_buffer).expect("JSON parsing failed at unescaping.");

    let obj: Value = serde_json::from_str(&json_buffer.to_owned())?;

    if let Some(searched_key) = std::env::args().nth(1) {
        log_key(&searched_key, &obj);

        return Ok(());
    }

    pretty_print_value(&obj);

    Ok(())
}
