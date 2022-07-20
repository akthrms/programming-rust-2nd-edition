use std::{error::Error, io};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("usage: http-get URL");
        return;
    }

    if let Err(err) = http_get_main(&args[1]) {
        eprint!("error: {}", err);
    }
}

fn http_get_main(url: &str) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        Err(format!("{}", response.status()))?;
    }

    let stdout = io::stdout();

    io::copy(&mut response, &mut stdout.lock())?;

    Ok(())
}
