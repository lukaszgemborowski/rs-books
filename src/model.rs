use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub first_name: String,
    pub last_name: String,
}

impl std::string::ToString for Author {
    fn to_string(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub isbn: Option<String>,
    pub author_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    authors: Vec<Author>,
    books: Vec<Book>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            authors: Vec::new(),
            books: Vec::new(),
        }
    }

    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let db = serde_json::from_reader(file)?;
        Ok(db)
    }

    pub fn add_author(&mut self, first_name: String, last_name: String) -> usize {
        self.authors.push(Author {
            first_name,
            last_name,
        });

        return self.authors.len() - 1;
    }

    pub fn add_book(&mut self, title: String, isbn: Option<String>, author_id: usize) {
        self.books.push(Book {
            name: title,
            isbn,
            author_id,
        });
    }

    pub fn author_list(&self) -> &Vec<Author> {
        return &self.authors;
    }

    pub fn book_list(&self) -> &Vec<Book> {
        return &self.books;
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }
}
