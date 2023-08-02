use crate::db::*;

use std::sync::Arc;
use tauri::State;

use prisma_client_rust::{Direction, QueryError};
type DbState<'a> = State<'a, Arc<PrismaClient>>;

#[tauri::command]
pub async fn get_chat(
    chat_id: String,
    db: DbState<'_>,
) -> Result<Option<conversation::Data>, QueryError> {
    db.conversation()
        .find_unique(conversation::id::equals(chat_id))
        .with(conversation::messages::fetch(vec![]))
        .exec()
        .await
        .map_err(|err| err)
}

#[tauri::command]
pub async fn create_chat(
    title: String,
    id: String,
    agent_id: String,
    db: DbState<'_>,
) -> Result<conversation::Data, QueryError> {
    db.conversation()
        .create(title, id, agent::id::equals(agent_id), vec![])
        .exec()
        .await
        .map_err(|err| err)
}

#[tauri::command]
pub async fn delete_chat(id: String, db: DbState<'_>) -> Result<conversation::Data, QueryError> {
    db.conversation()
        .delete(conversation::id::equals(id))
        .exec()
        .await
        .map_err(|err| err)
}

#[tauri::command]
pub async fn list_chats(
    agent_id: String,
    db: DbState<'_>,
) -> Result<Vec<conversation::Data>, QueryError> {
    db.conversation()
        .find_many(vec![conversation::agent::is(vec![agent::id::equals(
            agent_id,
        )])])
        .with(
            conversation::messages::fetch(vec![message::is_from_agent::equals(true)])
                .order_by(message::created_at::order(Direction::Desc))
                .take(1),
        )
        .exec()
        .await
        .map_err(|err| err)
}
