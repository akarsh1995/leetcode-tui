pub mod enums;
pub mod errors;
pub mod models;

use leetcode_core as api;
use libsql::{Builder, Connection};
pub use models::{question::DbQuestion, topic::DbTopic};
use std::path::PathBuf;
use std::sync::OnceLock;

pub static DB_CONN: OnceLock<Connection> = OnceLock::new();

pub(crate) fn get_conn() -> Connection {
    DB_CONN
        .get()
        .expect("Database connection is not initialized")
        .clone()
}

pub(crate) fn run_sync<F, R>(f: F) -> R
where
    F: std::future::Future<Output = R>,
{
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        tokio::task::block_in_place(|| handle.block_on(f))
    } else {
        tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime for DB operation")
            .block_on(f)
    }
}

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS questions (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    title_slug TEXT NOT NULL,
    difficulty TEXT NOT NULL,
    paid_only INTEGER NOT NULL,
    status TEXT,
    likes INTEGER NOT NULL DEFAULT 0,
    dislikes INTEGER NOT NULL DEFAULT 0,
    acceptance_rate REAL
);

CREATE TABLE IF NOT EXISTS topics (
    slug TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS question_topics (
    question_id INTEGER NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    topic_slug TEXT NOT NULL REFERENCES topics(slug) ON DELETE CASCADE,
    PRIMARY KEY (question_id, topic_slug)
);

CREATE INDEX IF NOT EXISTS idx_qt_topic ON question_topics(topic_slug);
";

pub fn init(db_path: Option<&PathBuf>) {
    DB_CONN.get_or_init(|| {
        run_sync(async {
            let db = if let Some(path) = db_path {
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                // Check if existing file is not a valid SQLite database (e.g. legacy native_db file)
                if path.exists() {
                    use std::io::Read;
                    let is_valid_sqlite = if let Ok(mut f) = std::fs::File::open(path) {
                        let mut magic = [0u8; 16];
                        f.read_exact(&mut magic).is_ok() && &magic == b"SQLite format 3\0"
                    } else {
                        false
                    };

                    if !is_valid_sqlite {
                        let backup_path = path.with_extension("db.native_db.bak");
                        log::warn!(
                            "Existing database at {} is not a valid SQLite database (legacy native_db format). Backing up to {} and creating a new SQLite database.",
                            path.display(),
                            backup_path.display()
                        );
                        let _ = std::fs::rename(path, backup_path);
                    }
                }

                Builder::new_local(path.to_str().expect("Invalid DB path"))
                    .build()
                    .await
                    .expect("Error while creating db connection.")
            } else {
                Builder::new_local(":memory:")
                    .build()
                    .await
                    .expect("Error while creating db connection in memory.")
            };

            let conn = db.connect().expect("Failed to connect to database");
            conn.execute_batch(SCHEMA)
                .await
                .expect("Failed to initialize database schema");

            conn
        })
    });
}
