use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum TakenItems {
    Table,
    Rounds,
    RoundsTotal,
    RoundsLeft,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TakenItems::Table)
                    .rename_column(TakenItems::Rounds, TakenItems::RoundsLeft)
                    //
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TakenItems::Table)
                    .add_column_if_not_exists(tiny_integer(TakenItems::RoundsTotal).borrow_mut())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TakenItems::Table)
                    .rename_column(TakenItems::RoundsLeft, TakenItems::Rounds)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TakenItems::Table)
                    .drop_column(TakenItems::RoundsTotal)
                    .to_owned(),
            )
            .await
    }
}
