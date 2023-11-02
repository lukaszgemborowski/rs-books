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
    pub owner_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    authors: Vec<Author>,
    books: Vec<Book>,
    owners: Vec<Owner>,
    default_owner: usize,
}

impl Database {
    pub fn new() -> Self {
        Database {
            authors: Vec::new(),
            books: Vec::new(),
            owners: Vec::new(),
            default_owner: 0,
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

    pub fn add_book(&mut self, title: String, isbn: Option<String>, author_id: usize) -> usize {
        self.books.push(Book {
            name: title,
            isbn: isbn,
            author_id: author_id,
            owner_id: self.default_owner,
        });

        return self.books.len() - 1;
    }

    pub fn author_list(&self) -> &Vec<Author> {
        return &self.authors;
    }

    pub fn book_list(&self) -> &Vec<Book> {
        return &self.books;
    }

    pub fn owner_list(&self) -> &Vec<Owner> {
        return &self.owners;
    }

    pub fn add_default_owner(&mut self, name: String) -> usize {
        self.owners.push(Owner {
            name: name,
        });

        self.default_owner = self.owners.len() - 1;

        return self.default_owner;
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }
}
