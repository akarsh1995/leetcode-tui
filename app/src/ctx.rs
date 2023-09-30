use core::question::Questions;
use core::topic::Topic;
use leetcode_db::Db;

pub struct Ctx {
    pub topic: Topic,
    pub question: Questions,
}

impl Ctx {
    pub(super) async fn new(db: &Db) -> Self {
        let t = Self {
            topic: Topic::new(db).await,
            question: Questions::new(),
        };
        t
    }
}
