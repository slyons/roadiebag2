#![allow(clippy::unused_async)]

use axum::debug_handler;
use loco_rs::prelude::*;
use crate::models::taken_items;
use crate::models::users;

#[debug_handler]
pub async fn get_current(
    State(ctx): State<AppContext>,
    auth: auth::JWT,
) -> Result<Json<Option<interface::TakenItem>>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(taken_items::Model::get_current(&ctx.db).await?)
}

pub async fn decrement_rounds(
    State(ctx): State<AppContext>,
    auth: auth::JWT,
) -> Result<Json<Option<interface::TakenItem>>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(taken_items::Model::decrement_rounds(&ctx.db).await?)
}

pub async fn mark_done(
    State(ctx): State<AppContext>,
    auth: auth::JWT,
) -> Result<()> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    taken_items::Model::mark_done(&ctx.db).await?;
    Ok(())
}

pub async fn get_random(
    State(ctx): State<AppContext>,
    auth: auth::JWT,
) -> Result<Json<interface::TakenItem>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    format::json(
        taken_items::Model::get_random(&ctx.db).await?
    )
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("taken")
        .add("/", get(get_current))
        .add("/", post(get_random))
        .add("/decrement", post(decrement_rounds))
        .add("/done", post(mark_done))
}
