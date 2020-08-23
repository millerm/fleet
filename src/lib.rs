pub mod fleet {
    use rusqlite::{ Connection, params, Error };

    #[derive(Debug)]
    pub struct Note {
        id: i32,
        content: String
    }

    pub fn insert(_content: &str) -> Result<(), Error> {
        let conn = Connection::open("fleet.db")?;

        conn.execute("CREATE TABLE IF NOT EXISTS note (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL
        )", params![]).unwrap();

        conn.execute(
            "INSERT INTO note (content) VALUES (?1)",
            params![_content.to_string()],
        )?;

        println!("Note added");

        return Ok(());
    }

    pub fn get_all() -> Result<(), Error> {
        let conn = Connection::open("fleet.db")?;
        let mut stmt = conn.prepare("SELECT id, content from note")?;

        let note_itr = stmt.query_map(params![], |row| {
            Ok(Note {
                id: row.get(0)?,
                content: row.get(1)?,
            })
        })?;

        for note in note_itr {
            let _n = note.unwrap();

            println!("#{}: {}", _n.id, _n.content);
        }

        Ok(())
    }

    pub fn delete_note(id: &i32) -> Result<(), Error> {
        let conn = Connection::open("fleet.db")?;

        conn.execute(
            "DELETE FROM note WHERE id = (?1)",
            params![id],
        )?;

        println!("Deleted note with id: {}", id);

        return Ok(());
    }
}
