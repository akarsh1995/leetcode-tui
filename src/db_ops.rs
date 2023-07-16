pub mod question;
pub mod topic_tag;

use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{prelude::*, sea_query::OnConflict};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
pub trait ModelUtils: Serialize + std::marker::Sized {
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity>
        + std::marker::Send
        + std::convert::From<Self::Model>;
    type Entity: EntityTrait;
    type Model: ModelTrait + DeserializeOwned;

    fn to_model(&self) -> Self::Model {
        let p = serde_json::to_string(self).unwrap();
        serde_json::from_str(p.as_str()).unwrap()
    }

    async fn to_db(&self, db: &DatabaseConnection) {
        let m: Self::ActiveModel = self.get_active_model();
        Self::Entity::insert(m)
            .on_conflict(Self::on_conflict())
            .exec(db)
            .await
            .unwrap();
    }

    async fn post_multi_insert(_db: &DatabaseConnection, _objects: Vec<Self>) {}

    async fn multi_insert(db: &DatabaseConnection, objects: Vec<Self>) {
        let mut v = vec![];
        for obj in &objects {
            let k: Self::ActiveModel = obj.clone().get_active_model();
            v.push(k);
        }
        Self::Entity::insert_many(v)
            .on_conflict(Self::on_conflict())
            .exec(db)
            .await
            .unwrap();
        Self::post_multi_insert(db, objects).await;
    }

    fn get_active_model(&self) -> Self::ActiveModel {
        self.to_model().into()
    }

    fn on_conflict() -> OnConflict;
}
