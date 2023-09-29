use core::topic::Topic;
use leetcode_db::Db;

pub struct Ctx {
    pub topic: Topic,
}

impl Ctx {
    pub(super) async fn new(db: &Db) -> Self {
        let t = Self {
            topic: Topic::new(db).await,
        };
        t
    }
}
