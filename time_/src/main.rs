use std::env::{self, current_dir};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use command line argument or default file
    let path = env::args().nth(1).unwrap_or_else(|| "content.txt".to_string());
    println!("Reading from {}", path);

    let book = Book::new(&path)?;
    println!("Book path: {}", book.path);

    // Example search usage
    let query = "maggot";
    println!("Contains '{}': {}", query, book.txt_search_bool(query)?);

    Ok(())
}

struct Book {
    path: String,
    content: File,
}

impl Book {
    fn new(file: &str) -> Result<Book, Box<dyn std::error::Error>> {
        // Build absolute path more safely
        let mut path = current_dir()?;
        path.push(file);

        let content = File::open(&path)?;

        Ok(Book {
            path: path.to_string_lossy().into_owned(),
            content,
        })
    }

    fn txt_search_bool(&self, query: &str) -> Result<bool, std::io::Error> {
        // Reset file position to start in case of multiple searches
        let buffer = BufReader::new(self.content.try_clone()?);
        Ok(buffer.lines().filter(|line| line.as_ref().unwrap().contains(query)).count() > 0)
    }
}