pub mod core {
    use rusqlite::{ Connection, params, Error };
    use std::path::Path;
    use std::process;
    extern crate dirs;
    use std::ffi::OsString;

    #[derive(Debug)]
    pub struct Note {
        id: i32,
        content: String
    }

    pub fn insert(_content: &str) -> Result<(), Error> {
        let home = match dirs::home_dir() {
            None => OsString::new(),
            Some(dir) => dir.into_os_string(),
        };
        let path = Path::new(&home).join("fleet.db");
        let conn = Connection::open(path)?;

        conn.execute("CREATE TABLE IF NOT EXISTS note (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL
        )", params![])?;

        conn.execute(
            "INSERT INTO note (content) VALUES (?1)",
            params![_content.to_string()],
        )?;

        println!("Note added with content: {}", _content.to_string());

        return Ok(());
    }

    pub fn get_all() -> Result<(), Error> {
        let home = match dirs::home_dir() {
            None => OsString::new(),
            Some(dir) => dir.into_os_string(),
        };
        let path = Path::new(&home).join("fleet.db");
        let conn = Connection::open(path)?;
        let mut stmt = conn.prepare("SELECT id, content from note")?;
        let note_itr = stmt.query_map(params![], |row| {
            Ok(Note {
                id: row.get(0)?,
                content: row.get(1)?,
            })
        })?;

        for note in note_itr {
            let _n = note.unwrap_or_else(|err| {
                eprintln!("Problem reading note! Error: {}", err);
                process::exit(1);
            });

            println!("#{}: {}", _n.id, _n.content);
        }

        Ok(())
    }

    pub fn delete_note(id: &i32) -> Result<(), Error> {
        let home = match dirs::home_dir() {
            None => OsString::new(),
            Some(dir) => dir.into_os_string(),
        };
        let path = Path::new(&home).join("fleet.db");
        let conn = Connection::open(path)?;

        conn.execute(
            "DELETE FROM note WHERE id = (?1)",
            params![id],
        )?;

        println!("Deleted note with id: {}", id);

        return Ok(());
    }
}
