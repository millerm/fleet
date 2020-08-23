pub mod fleet {
    use chrono::prelude::*;
    use serde::{Serialize, Deserialize, Serializer};
    use std::error::Error;
    use sled::{Config, IVec, Result};
    use bincode::{serialize, deserialize};

    #[derive(Debug)]
    #[derive(Default, Serialize, Deserialize)]
    pub struct NoteConfig {
        pub id: String,
        pub content: String,
    }

    pub fn insert(table: String, note: NoteConfig) {
        let tree = sled::open(table).expect("Error opening");
        let bytes = bincode::serialize(&note).expect("error serializing");
        // TODO: generate Key
        let k = b"key";

        tree.insert(&k, bytes);

        println!("successful insert");
    }

    pub fn create_note(table: &str, content: &str) {
        // TODO: Generate ID
        let new_note = NoteConfig {
            id: String::from("1"),
            content: content.to_string(),
        };

        insert(table.to_string(), new_note);
    }
}
