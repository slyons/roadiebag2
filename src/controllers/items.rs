#![allow(clippy::unused_async)]

use axum::extract::Query;
use loco_rs::prelude::*;
use crate::models::users;
use crate::models::items;

#[axum::debug_handler]
pub async fn create(State(ctx): State<AppContext>,
                    auth: auth::JWT,
                    Json(create): Json<interface::CreateUpdateItem>
) -> Result<Json<interface::Item>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let item = items::Model::create(
        &ctx.db,
        create
    ).await?;

    format::json(item.into())
}

#[axum::debug_handler]
async fn read(State(ctx): State<AppContext>,
                  auth: auth::JWT,
                  Path(id): Path<i32>) -> Result<Json<interface::Item>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    format::json(
        items::Model::find_by_id(&ctx.db, id)
            .await?
            .into()
    )
}

#[axum::debug_handler]
pub async fn update(State(ctx): State<AppContext>,
                auth: auth::JWT,
                Path(id): Path<i32>,
                Json(update): Json<interface::CreateUpdateItem>
) -> Result<Json<interface::Item>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    format::json(items::Model::update(
                &ctx.db,
                id,
                update
            )
           .await?
           .into())
}

#[axum::debug_handler]
pub async fn delete_item(State(ctx): State<AppContext>,
                    auth: auth::JWT,
                    Path(id): Path<i32>) -> Result<()> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    items::Model::delete(&ctx.db, id).await?;
    Ok(())
}

#[axum::debug_handler]
pub async fn list_items(State(ctx): State<AppContext>,
                        filter: Option<Query<interface::ItemFilter>>,
                        auth: auth::JWT,
) -> Result<Json<interface::ItemPage>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(items::Model::list(&ctx.db, filter.map(|f| f.0)).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("items")
        .add("/", post(create))
        .add("/:id", get(read))
        .add("/:id", post(update))
        .add("/:id", delete(delete_item))
}
