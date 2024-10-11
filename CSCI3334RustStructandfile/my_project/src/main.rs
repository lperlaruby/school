use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) -> Result<(), Error> {
    let mut file = File::create(filename)?;
    
    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year)?;
    }
    
    Ok(())
}

fn load_books(filename: &str) -> Result<Vec<Book>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year = parts[2].parse::<u16>().unwrap_or(0); // Handle parsing errors
            books.push(Book { title, author, year });
        }
    }

    Ok(books)
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    if save_books(&books, "books.txt").is_ok() {
        println!("Books saved to file.");
    } else {
        println!("Failed to save books.");
    }

    match load_books("books.txt") {
        Ok(loaded_books) => {
            println!("Loaded books:");
            for book in loaded_books {
                println!("{} by {}, published in {}", book.title, book.author, book.year);
            }
        }
        Err(_) => {
            println!("Failed to load books.");
        }
    }
}