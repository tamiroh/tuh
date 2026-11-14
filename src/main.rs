use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn rooms(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut rooms = Vec::new();
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            rooms.push(path);
        }
    }
    rooms.sort();
    Ok(rooms)
}

fn main() -> io::Result<()> {
    let mut current = std::env::current_dir()?;
    let mut children = rooms(&current)?;

    println!("Enter a number to move / .. to go up / q to quit");
    loop {
        println!("\n💡 {}", current.display());
        for (index, child) in children.iter().enumerate() {
            println!(
                "  {}. 🚪 {}",
                index + 1,
                child.file_name().unwrap_or_default().to_string_lossy()
            );
        }
        if children.is_empty() {
            println!("  No subdirectories.");
        }
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 || input.trim() == "q" {
            println!("🌑 Lights off: {}", current.display());
            break;
        }

        let destination = match input.trim() {
            ".." => current.parent().map(Path::to_path_buf),
            number => number
                .parse::<usize>()
                .ok()
                .and_then(|number| number.checked_sub(1))
                .and_then(|index| children.get(index).cloned()),
        };
        let Some(destination) = destination else {
            println!("No such room. Enter a listed number, .., or q.");
            continue;
        };

        match destination
            .canonicalize()
            .and_then(|path| rooms(&path).map(|children| (path, children)))
        {
            Ok((path, next_children)) => {
                println!("🌑 Lights off: {}", current.display());
                current = path;
                children = next_children;
            }
            Err(error) => println!("Could not enter room: {error}"),
        }
    }
    Ok(())
}
