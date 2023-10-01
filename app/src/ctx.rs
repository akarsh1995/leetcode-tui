use core::popup::Popup;
use core::question::Questions;
use core::topic::Topic;
use leetcode_db::Db;

pub struct Ctx {
    pub topic: Topic,
    pub question: Questions,
    pub popup: Popup,
}

impl Ctx {
    pub(super) async fn new(db: &Db) -> Self {
        Self {
            topic: Topic::new(db).await,
            question: Questions::default(),
            popup: Default::default(),
        }
    }
}
