use domain_info::process;

use std::env;

fn main() -> Result<(), String> {
    // Retrieve url from context parameter
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(format!("Unexpected number of parameters: {}", args.len()))
    }
    let domain = &args[1];

    let result = process::process(domain)?;

    println!("{}", serde_json::to_string_pretty(&result).unwrap());

    Ok(())
}