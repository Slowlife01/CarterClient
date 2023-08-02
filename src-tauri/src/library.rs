use std::sync::Arc;
use std::{fs, path::PathBuf};
use tauri::api::path;

use crate::db::*;

type Ctx = Arc<PrismaClient>;

pub async fn init_db() -> Result<Ctx, ()> {
    let data_dir = path::data_dir()
        .unwrap_or_else(|| PathBuf::from("./"))
        .join("carterclient");

    let db_path = data_dir.join("library.db");

    fs::create_dir_all(data_dir).unwrap();
    if !db_path.exists() {
        fs::File::create(&db_path).unwrap();
    }

    let db = new_client_with_url(format!("file:{}", db_path.to_str().unwrap()).as_str())
        .await
        .unwrap();

    #[cfg(debug_assertions)]
    db._db_push().await.unwrap();

    #[cfg(not(debug_assertions))]
    db._migrate_deploy().await.unwrap();

    Ok(Arc::new(db))
}