mod model;
mod interactive;

fn main() {
    let mut db = model::Database::load(std::path::Path::new("db.json")).unwrap_or_else(|_| {
        model::Database::new()
    });

    loop {
        if let interactive::Interaction::Exit = interactive::interact(&mut db) {
            break;
        }
    }

    db.save(std::path::Path::new("db.json")).unwrap();
}