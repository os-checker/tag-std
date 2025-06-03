use super::Data;
use crate::Result;
use eyre::Context;
use rusqlite::{Connection, named_params};

pub struct Database {
    conn: Connection,
}

const CREATE: &str = "
CREATE TABLE IF NOT EXISTS hir (
  hash1 INTEGER NOT NULL,
  hash2 INTEGER NOT NULL,
  tool_attrs TEXT NOT NULL,
  def_path TEXT NOT NULL,
  function TEXT NOT NULL,
  timestamp TEXT NOT NULL,
  PRIMARY KEY (hash1, hash2)
) STRICT;
";

impl Database {
    pub fn new(path: &str) -> Result<Database> {
        let conn = Connection::open(path).with_context(|| format!("Failed to open {path}"))?;
        conn.execute(CREATE, ()).with_context(|| format!("Failed to execute sql:\n{CREATE}"))?;
        Ok(Database { conn })
    }

    pub fn save_data(&mut self, iter: impl IntoIterator<Item = Data>) -> Result<()> {
        const UPSERT: &str = "
INSERT OR REPLACE INTO hir (hash1, hash2, tool_attrs, def_path, function, timestamp)
VALUES (:hash1, :hash2, :tool_attrs, :def_path, :function, :timestamp)
";
        let mut stmt = self.conn.prepare(UPSERT)?;

        let timestamp = jiff::Timestamp::now();
        for data in iter {
            let tool_attrs = serde_json::to_string(data.func.tool_attrs.as_slice())?;
            stmt.execute(named_params! {
                ":hash1": data.hash.hash1.cast_signed(),
                ":hash2": data.hash.hash2.cast_signed(),
                ":tool_attrs": tool_attrs,
                ":def_path": data.func.def_path,
                ":function": data.func.function,
                ":timestamp": timestamp,
            })?;
        }

        stmt.finalize()?;

        Ok(())
    }
}

#[test]
fn test_db() -> Result<()> {
    use super::{Func, PrimaryKey};
    let mut db = Database::new("a.sqlite3")?;
    db.save_data([Data {
        hash: PrimaryKey { hash1: 1, hash2: 2 },
        func: Func {
            tool_attrs: vec!["Safety".to_owned()],
            def_path: "a::b".to_owned(),
            function: "fn f() {}".to_owned(),
        },
    }])
}
