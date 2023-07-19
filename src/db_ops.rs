pub mod question;
pub mod topic_tag;

use sea_orm::prelude::async_trait::async_trait;
use sea_orm::InsertResult;
use sea_orm::{prelude::*, sea_query::OnConflict};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::AppResult;

#[async_trait]
pub trait ModelUtils: Serialize + std::marker::Sized {
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity>
        + std::marker::Send
        + std::convert::From<Self::Model>;
    type Entity: EntityTrait;
    type Model: ModelTrait + DeserializeOwned;

    fn to_model(&self) -> AppResult<Self::Model> {
        let p = serde_json::to_string(self)?;
        Ok(serde_json::from_str(p.as_str())?)
    }

    async fn to_db(&self, db: &DatabaseConnection) -> AppResult<InsertResult<Self::ActiveModel>> {
        let m: Self::ActiveModel = self.get_active_model()?;
        Ok(Self::Entity::insert(m)
            .on_conflict(Self::on_conflict())
            .exec(db)
            .await?)
    }

    async fn post_multi_insert(_db: &DatabaseConnection, _objects: Vec<Self>) -> AppResult<()> {
        Ok(())
    }

    async fn multi_insert(db: &DatabaseConnection, objects: Vec<Self>) -> AppResult<()> {
        let mut v = vec![];
        for obj in &objects {
            let k: Self::ActiveModel = obj.clone().get_active_model()?;
            v.push(k);
        }
        if !v.is_empty() {
            Self::Entity::insert_many(v)
                .on_conflict(Self::on_conflict())
                .exec(db)
                .await?;
        }
        Self::post_multi_insert(db, objects).await?;
        Ok(())
    }

    fn get_active_model(&self) -> AppResult<Self::ActiveModel> {
        let model = self.to_model()?;
        Ok(model.into())
    }

    fn on_conflict() -> OnConflict;
}
