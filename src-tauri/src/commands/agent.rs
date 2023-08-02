use crate::db::*;

use std::sync::Arc;
use tauri::State;

use prisma_client_rust::QueryError;
type DbState<'a> = State<'a, Arc<PrismaClient>>;

#[tauri::command]
pub async fn create_agent(
    name: String,
    key: String,
    db: DbState<'_>,
) -> Result<agent::Data, QueryError> {
    db.agent()
        .create(name, key, vec![])
        .exec()
        .await
        .map_err(|err| err)
}

#[tauri::command]
pub async fn update_agent(
    id: String,
    name: String,
    key: String,
    db: DbState<'_>,
) -> Result<agent::Data, ()> {
    db.agent()
        .update(
            agent::id::equals(id),
            vec![agent::SetParam::SetName(name), agent::SetParam::SetKey(key)],
        )
        .exec()
        .await
        .map_err(|_| ())
}

#[tauri::command]
pub async fn remove_agent(id: String, db: DbState<'_>) -> Result<agent::Data, QueryError> {
    db.agent()
        .delete(agent::id::equals(id))
        .exec()
        .await
        .map_err(|err| err)
}

#[tauri::command]
pub async fn list_agents(db: DbState<'_>) -> Result<Vec<agent::Data>, ()> {
    db.agent().find_many(vec![]).exec().await.map_err(|_| ())
}

#[tauri::command]
pub async fn set_selected_agent(id: String, db: DbState<'_>) -> Result<agent::Data, ()> {
    let agent = db
        .agent()
        .find_first(vec![agent::id::equals(id.clone())])
        .exec()
        .await;

    if let Ok(Some(agent)) = agent {
        db.agent()
            .update(
                agent::id::equals(id.clone()),
                vec![agent::SetParam::SetIsSelected(true)],
            )
            .exec()
            .await
            .map_err(|_| ())?;

        db.agent()
            .update_many(
                vec![agent::id::not(id), agent::is_selected::equals(true)],
                vec![agent::SetParam::SetIsSelected(false)],
            )
            .exec()
            .await
            .map_err(|_| ())?;

        return Ok(agent);
    }

    Err(())
}

#[tauri::command]
pub async fn get_selected_agent(db: DbState<'_>) -> Result<Option<agent::Data>, ()> {
    let agent = db
        .agent()
        .find_first(vec![agent::is_selected::equals(true)])
        .exec()
        .await;

    if let Ok(Some(agent)) = agent {
        return Ok(Some(agent));
    }

    Ok(None)
}
