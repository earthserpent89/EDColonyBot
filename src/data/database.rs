// This file handles database interactions, exporting functions for connecting to and querying the database.

use rusqlite::{params, Connection, Result};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let connection = Connection::open(db_path)?;
        Ok(Database { connection })
    }

    pub fn create_table(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS construction_sites (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                location TEXT NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_site(&self, name: &str, location: &str, status: &str) -> Result<()> {
        self.connection.execute(
            "INSERT INTO construction_sites (name, location, status) VALUES (?1, ?2, ?3)",
            params![name, location, status],
        )?;
        Ok(())
    }

    pub fn get_sites(&self) -> Result<Vec<(i32, String, String, String)>> {
        let mut stmt = self.connection.prepare("SELECT id, name, location, status FROM construction_sites")?;
        let site_iter = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut sites = Vec::new();
        for site in site_iter {
            sites.push(site?);
        }
        Ok(sites)
    }
}