use chrono::Utc;
use interface::TakenItem;
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::{
    entity::prelude::*,
    sea_query::{Alias, Query},
    ActiveValue, JoinType, QuerySelect, TransactionTrait,
};

use super::_entities::items;
pub use super::_entities::taken_items::{self, ActiveModel, Entity, Model};

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        {
            let mut this = self;

            if insert {
                this.created_at = ActiveValue::Set(Utc::now().naive_utc());
            }
            this.updated_at = ActiveValue::Set(Utc::now().naive_utc());
            Ok(this)
        }
    }
}

impl Into<interface::TakenItem> for Model {
    fn into(self) -> TakenItem {
        TakenItem {
            created_at: self.created_at,
            updated_at: self.updated_at,
            id: self.id,
            item_id: self.item_id,
            rounds_left: self.rounds_left,
            done: self.done,
            rounds_total: self.rounds_total,
        }
    }
}

impl Model {
    pub async fn get_current(db: &DatabaseConnection) -> ModelResult<Option<interface::TakenItem>> {
        Ok(taken_items::Entity::find()
            .filter(taken_items::Column::Done.eq(false))
            .one(db)
            .await?
            .map(Into::into))
    }

    pub async fn decrement_rounds(
        db: &DatabaseConnection,
    ) -> ModelResult<Option<interface::TakenItem>> {
        let current_item = Model::get_current(db).await?;
        if let Some(itm) = current_item {
            let new_round_count = itm.rounds_left - 1;
            let done = new_round_count <= 0;
            let model = ActiveModel {
                id: ActiveValue::Set(itm.id),
                rounds_left: ActiveValue::Set(new_round_count),
                done: ActiveValue::Set(done),
                ..Default::default()
            }
            .update(db)
            .await?;
            Ok(Some(model.into()))
        } else {
            Ok(current_item)
        }
    }

    pub async fn mark_done(db: &DatabaseConnection) -> ModelResult<()> {
        let current_item = Model::get_current(db).await?;
        if let Some(itm) = current_item {
            let model = ActiveModel {
                id: ActiveValue::Set(itm.id),
                done: ActiveValue::Set(true),
                ..Default::default()
            }
            .update(db)
            .await?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub async fn get_random(db: &DatabaseConnection) -> ModelResult<interface::TakenItem> {
        let existing = Model::get_current(db).await?;
        if let Some(ext) = existing {
            tracing::info!("Current item is {:?}", ext);
            Ok(ext)
        } else {
            let txn = db.begin().await?;
            let item_uses = items::Entity::find()
                .left_join(taken_items::Entity)
                .group_by(items::Column::Id)
                .to_owned()
                .having(
                    Expr::expr(
                        Expr::case(
                            items::Column::Infinite.eq(false),
                            items::Column::Quantity.into_expr().sub(
                                Expr::expr(taken_items::Column::Id.into_expr().count()).if_null(0),
                            ),
                        )
                        .finally(1),
                    )
                    .gte(1),
                );
            //.column(items::Column::Id);
            let item_count = item_uses.clone().count(&txn).await?;
            tracing::info!("Item count is {}", item_count);
            let item_offset = rand::random::<u64>().clamp(0, item_count - 1);
            tracing::info!("Selecting offset {}", item_offset);
            //let item_id = item_uses.offset(item_offset).into_tuple().one(&txn)
            //    .await?
            //    .expect(&format!("Item with offset {} returned None", item_offset));
            let item = item_uses
                .offset(item_offset)
                .one(&txn)
                .await?
                .expect(&format!("Item with offset {} returned None", item_offset));
            let total_rounds = rand::random::<i16>().clamp(1, 6);
            let model = ActiveModel {
                item_id: ActiveValue::Set(item.id),
                rounds_left: ActiveValue::Set(total_rounds),
                rounds_total: ActiveValue::Set(total_rounds),
                done: ActiveValue::Set(false),
                ..Default::default()
            }
            .insert(&txn)
            .await?;

            txn.commit().await?;
            Ok(model.into())
        }
    }
}
