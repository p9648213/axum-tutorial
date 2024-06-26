use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;
use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use sea_orm::{prelude::DateTime, DatabaseConnection, Set};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTime>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<DateTime>>,
}

pub async fn partial_update(
    Path(id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let mut db_task = if let Some(task) = Tasks::find_by_id(id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(priority) = request_task.priority {
        db_task.priority = Set(priority);
    }

    if let Some(description) = request_task.description {
        db_task.description = Set(description);
    }

    if let Some(title) = request_task.title {
        db_task.title = Set(title);
    }

    if let Some(completed_at) = request_task.completed_at {
        db_task.completed_at = Set(completed_at);
    }

    if let Some(deleted_at) = request_task.deleted_at {
        db_task.deleted_at = Set(deleted_at);
    }

    Tasks::update(db_task)
        .filter(tasks::Column::Id.eq(id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
