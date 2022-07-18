use std::{
    error::Error,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::PathBuf,
};

fn main() {
    let result = grep_main();

    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line_result in reader.lines() {
        let line = line_result?;

        if line.contains(target) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn grep_main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);

    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE...")?,
    };

    let files = args.map(PathBuf::from).collect::<Vec<PathBuf>>();

    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?
        }
    }

    Ok(())
}
