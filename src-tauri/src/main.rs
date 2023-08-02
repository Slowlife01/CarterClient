// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub(crate) mod carter;
pub(crate) mod commands;
pub(crate) mod library;

#[allow(warnings, unused)]
pub(crate) mod db;

use std::sync::Arc;
use tauri::AppHandle;
use tauri::{Manager, State};

use crate::carter::*;
use crate::db::*;
use crate::library::*;

type DbState<'a> = State<'a, Arc<PrismaClient>>;

fn get_carter(key: String, user_id: String) -> Carter {
    let carter = Carter::new(key, user_id);
    carter
}

#[tauri::command]
async fn chat(chat_id: String, agent_key: String, input: String, db: DbState<'_>) -> Result<ChatResponse, ()> {
        let carter = get_carter(agent_key, chat_id.clone());
        let response = carter.chat(&input).await.unwrap();

        db._batch(vec![
            db.message().create(
                conversation::id::equals(chat_id.clone()),
                input,
                vec![message::is_from_agent::set(false)],
            ),
            db.message().create(
                conversation::id::equals(chat_id),
                response.output.as_ref().unwrap().text.to_string(),
                vec![],
            ),
        ])
        .await
        .unwrap();

        Ok(response)
}

#[tauri::command]
async fn opener(chat_id: String, agent_key: String, db: DbState<'_>) -> Result<OpenerResponse, ()> {
        let carter = get_carter(agent_key, chat_id.clone());
        let response = carter.opener().await.unwrap();

            db.message().create(
                conversation::id::equals(chat_id),
                response.output.as_ref().unwrap().text.to_string(),
                vec![],
            )
.exec()
        .await
        .unwrap();

        Ok(response)

}

#[tauri::command]
fn is_db_ready(handle: AppHandle) -> bool {
    handle.try_state::<Arc<PrismaClient>>().is_some()
}

#[tokio::main]
async fn main() -> tauri::Result<()> {
    let db = init_db().await.unwrap();

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            is_db_ready,
            // Agents
            commands::agent::set_selected_agent,
            commands::agent::get_selected_agent,
            commands::agent::create_agent,
            commands::agent::update_agent,
            commands::agent::remove_agent,
            commands::agent::list_agents,
            // Chats
            commands::chat::get_chat,
            commands::chat::create_chat,
            commands::chat::delete_chat,
            commands::chat::list_chats,
            // Carter API
            chat,
            opener,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
