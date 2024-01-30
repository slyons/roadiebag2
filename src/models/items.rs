use chrono::Utc;
use sea_orm::entity::prelude::*;
pub use super::_entities::items::{self, Entity, ActiveModel, Model};
use loco_rs:: {
    model::{ModelError, ModelResult},
    validation,
    validator::Validate,
};
use sea_orm::{entity::prelude::*, ActiveValue, DatabaseConnection, DbErr, TransactionTrait, QuerySelect, Paginator, QueryOrder};
use sea_orm::sea_query::{*, extension::postgres::PgExpr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Validate, Deserialize)]
pub struct ModelValidator {
    #[validate(length(min=1, message="Name must be at least 1 character long"))]
    pub name: String,
    #[validate(range(min=1))]
    pub quantity: i32,
    #[validate(range(min=0, max=3))]
    pub size: i16
}

impl From<&ActiveModel> for ModelValidator {
    fn from(value: &ActiveModel) -> Self {
        Self {
            name: value.name.as_ref().to_string(),
            quantity: *value.quantity.as_ref(),
            size: *value.size.as_ref()
        }
    }
}

impl Into<interface::Item> for Model {
    fn into(self) -> interface::Item {
        interface::Item {
            created_at: self.created_at,
            updated_at: self.updated_at,
            name: self.name,
            description: self.description,
            id: self.id,
            size: interface::ItemSize::from_repr(self.size).unwrap(),
            infinite: self.infinite,
            quantity: self.quantity
        }
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
        where C: ConnectionTrait
    {
        {
            self.validate()?;
            let mut this = self;

            if insert {
                this.created_at = ActiveValue::Set(Utc::now().naive_utc());
            }
            this.updated_at = ActiveValue::Set(Utc::now().naive_utc());
            Ok(this)
        }
    }
}

impl Model {
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let item = items::Entity::find()
            .filter(items::Column::Id.eq(id))
            .one(db)
            .await?;
        item.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn create(db: &DatabaseConnection, create: interface::CreateUpdateItem) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let item = items::ActiveModel {
            name: ActiveValue::Set(create.name),
            description: ActiveValue::Set(create.description),
            quantity: ActiveValue::Set(create.quantity),
            size: ActiveValue::Set(create.size as i16),
            infinite: ActiveValue::Set(create.infinite),
            ..Default::default()
        }
            .insert(&txn)
            .await?;

        txn.commit().await?;
        Ok(item)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update: interface::CreateUpdateItem) -> ModelResult<Self> {
        let txn = db.begin().await?;

        if items::Entity::find()
            .filter(items::Column::Id.eq(id))
            .one(&txn)
            .await?
            .is_none()
        {
            return Err(ModelError::EntityNotFound {});
        }

        let item = items::ActiveModel {
            name: ActiveValue::Set(update.name),
            description: ActiveValue::Set(update.description),
            quantity: ActiveValue::Set(update.quantity),
            size: ActiveValue::Set(update.size as i16),
            infinite: ActiveValue::Set(update.infinite),
            id: ActiveValue::Set(id),
            ..Default::default()
        }
            .update(&txn)
            .await?;
        txn.commit().await?;
        Ok(item)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> ModelResult<()> {
        let txn = db.begin().await?;

        if items::Entity::find()
            .filter(items::Column::Id.eq(id))
            .one(&txn)
            .await?
            .is_none()
        {
            return Err(ModelError::EntityNotFound {});
        }

        let item = items::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        }
            .delete(&txn)
            .await?;

        txn.commit().await?;
        Ok(())
    }

    pub async fn list(db: &DatabaseConnection, filter: Option<interface::ItemFilter>) -> ModelResult<interface::ItemPage> {
        let filter = filter.unwrap_or_default();

        let mut query = items::Entity::find()
            .order_by_desc(items::Column::Id);
        if let Some(mut name) = filter.name {
            if !name.contains("%") {
                name = name + "%";
            }
            query = query.filter(Expr::col(items::Column::Name).ilike(name));
        }

        if let Some(mut description) = filter.description {
            if !description.contains("%") {
                description = description + "%";
            }
            query = query.filter(Expr::col(items::Column::Description).ilike(description));
        }

        if let Some(size) = filter.size {
            let item_size = size as i16;
            query = query.filter(items::Column::Size.eq(item_size));
        }

        if let Some(infinite) = filter.infinite {
            query = query.filter(items::Column::Infinite.eq(infinite));
        }

        let page_size = if filter.page_size > 0 {
            filter.page_size
        } else {
            50
        };

        let paginator = query.paginate(db, page_size);
        let items_and_pages = paginator.num_items_and_pages().await?;
        let items = paginator
            .fetch_page(filter.page_num)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(interface::ItemPage {
            items: items,
            page_num: filter.page_num,
            page_size: page_size,
            total_pages: items_and_pages.number_of_pages,
            total_results: items_and_pages.number_of_items
        })
    }
}

impl ActiveModel {
    pub fn validate(&self) -> Result<(), DbErr> {
        let validator: ModelValidator = self.into();
        validator
            .validate()
            .map_err(|e| validation::into_db_error(&e))
    }
}