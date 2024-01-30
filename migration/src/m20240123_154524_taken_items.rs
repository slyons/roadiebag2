use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(TakenItems::Table)
                    .col(pk_auto(TakenItems::Id).borrow_mut())
                    .col(integer(TakenItems::ItemId).borrow_mut())
                    .col(tiny_integer(TakenItems::Rounds).borrow_mut())
                    .col(bool(TakenItems::Done).default(false).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-taken_items-items")
                            .from(TakenItems::Table, TakenItems::ItemId)
                            .to(Items::Table, Items::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TakenItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TakenItems {
    Table,
    Id,
    ItemId,
    Rounds,
    Done,
    
}


#[derive(DeriveIden)]
enum Items {
    Table,
    Id,
}
