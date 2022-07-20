use std::{
    fs,
    io::{self, Write},
    path::Path,
};

fn main() {
    if let Err(err) = copy_main() {
        write!(io::stderr(), "error: {}", err).unwrap();
    }
}

fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.is_dir() {
        fs::create_dir(dst)?;
    }

    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }

    Ok(())
}

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[cfg(not(unix))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!("can't copy symbolic link: {}", src.as_ref().display()),
    ))
}

fn copy_to(src: &Path, src_type: &fs::FileType, dst: &Path) -> io::Result<()> {
    if src_type.is_file() {
        fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else if src_type.is_symlink() {
        let target = src.read_link()?;
        symlink(target, dst)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("don't know how to copy: {}", src.display()),
        ));
    }

    Ok(())
}

fn copy_into<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    match src.file_name() {
        Some(file_name) => {
            let metadata = src.metadata()?;
            copy_to(src, &metadata.file_type(), &dst.join(file_name))?;
        }
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("can't copy nameless directory: {}", src.display()),
            ));
        }
    }

    Ok(())
}

fn dwim_copy<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if dst.is_dir() {
        copy_into(src, dst)
    } else {
        let metadata = src.metadata()?;
        copy_to(src, &metadata.file_type(), dst)
    }
}

fn copy_main() -> io::Result<()> {
    let args = std::env::args_os().collect::<Vec<_>>();

    if args.len() < 3 {
        println!("Usage: copy FILE... DESTINATION");
    } else if args.len() == 3 {
        dwim_copy(&args[1], &args[2])?;
    } else {
        let dst = Path::new(&args[args.len() - 1]);

        if !dst.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("target {} is not a directory", dst.display()),
            ));
        }

        for i in 1..args.len() - 1 {
            copy_into(&args[i], dst)?;
        }
    }

    Ok(())
}
