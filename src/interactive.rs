use crate::model;
use dialoguer::Input;
use dialoguer::Confirm;
use dialoguer::FuzzySelect;

fn user_input(prompt: &str) -> dialoguer::Result<String> {
    Input::<String>::new()
        .with_prompt(prompt)
        .interact()
}

fn user_confirm(prompt: &str) -> dialoguer::Result<bool> {
    Confirm::new()
        .with_prompt(prompt)
        .interact()
}

fn user_fuzzy<T>(prompt: &str, items: &T) -> dialoguer::Result<usize> {
    FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(items)
        .interact()
}

fn add_author(db: &mut model::Database) -> dialoguer::Result<usize> {
    return user_input("First name").and_then(|first_name| {
        user_input("Last name").map(|last_name| {
            return db.add_author(first_name, last_name)
        })
    });
}

fn ask_isbn() -> dialoguer::Result<Option<String>> {
    let has_isbn = user_confirm("Add ISBN?");

    match has_isbn {
        Ok(true) => user_input("ISBN").map(|isbn| Some(isbn)),
        Ok(false) => Ok(None),
        Err(e) => Err(e),
    }
}

fn add_book(db: &mut model::Database) -> dialoguer::Result<()> {
    let title = user_input("Title")?;

    let author_selection = user_fuzzy("Author", &db.author_list())?;

    let author_index = match author_selection {
        Some(index) => index,
        None        => add_author(db)
    };

    match ask_isbn()? {
        Some(isbn)  => db.add_book(title, Some(isbn), author_index),
        None        => db.add_book(title, None, author_index),
    };

    Ok(())
}

fn list_books(db: &model::Database) {
    struct BookAuthor<'a> {
        book: &'a model::Book,
        db: &'a model::Database,
    }

    impl<'a> std::string::ToString for BookAuthor<'a> {
        fn to_string(&self) -> String {
            format!("{} ({})", self.book.name, self.db.author_list()[self.book.author_id].to_string())
        }
    }

    let mut book_authors: Vec<BookAuthor> = Vec::new();

    for book in db.book_list() {
        book_authors.push(BookAuthor {
            book: &book,
            db: &db,
        });
    }

    let _ = FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .default(0)
        .items(&book_authors)
        .interact()
        .unwrap();
}

pub enum Interaction {
    Continue,
    Exit
}

pub fn interact(db: &mut model::Database) -> Interaction {
    let commands = vec![
        "Add book",
        "Add author",
        "List books",
        "List authors",
        "Quit",
    ];
    
    let selection = FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .default(0)
        .items(&commands)
        .interact()
        .unwrap();

    if selection == 0 {
        add_book(db);
    } else if selection == 1 {
        add_author(db);
    } else if selection == 2 {
        list_books(db);
    } else if selection == 3 {
    } else if selection == 4 {
        return Interaction::Exit;
    }

    return Interaction::Continue;
}