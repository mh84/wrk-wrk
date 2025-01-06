mod task;

pub use task::Task;

use std::fs;

use anyhow::Result;
use futures_core::future::BoxFuture;
use sqlx::{
    error::BoxDynError,
    migrate::{Migration, MigrationSource, MigrationType, Migrator},
    sqlite::SqlitePool,
    Pool, Sqlite,
};
use tauri::{AppHandle, Manager};

#[derive(Debug)]
pub struct Migrations(pub Vec<Migration>);

impl MigrationSource<'static> for Migrations {
    fn resolve(self) -> BoxFuture<'static, std::result::Result<Vec<Migration>, BoxDynError>> {
        Box::pin(async move { Ok(self.0) })
    }
}

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> Result<Self> {
        let app_dir = app_handle
            .path()
            .app_config_dir()
            .expect("failed to get app dir");

        fs::create_dir_all(&app_dir)?;

        let db_path = app_dir.join("wrkwrk.db");

        let connection_options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(connection_options).await?;

        Self::migrate(&pool).await?;

        Ok(Self { pool })
    }

    async fn migrate(pool: &SqlitePool) -> Result<()> {
        let migrations = vec![Migration::new(
            1,
            "add tasks table".into(),
            MigrationType::ReversibleUp,
            r#"
                CREATE TABLE IF NOT EXISTS tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    task_kind INTEGER NOT NULL,
                    finished INTEGER NOT NULL
                );

                INSERT INTO tasks (name, task_kind, finished)
                VALUES ('Pause', 0, 0);
            "#
            .into(),
            true,
        )];

        let migrations = Migrations(migrations);

        let migrator = Migrator::new(migrations).await?;
        migrator.run(pool).await?;

        Ok(())
    }
}

pub struct PoolProvider(pub Pool<Sqlite>);
