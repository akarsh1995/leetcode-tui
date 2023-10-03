use core::popup::{Popup, SelectPopup};
use core::question::Questions;
use core::topic::Topic;
use leetcode_db::Db;

pub struct Ctx {
    pub topic: Topic,
    pub question: Questions,
    pub popup: Popup,
    pub select_popup: SelectPopup<String>,
}

impl Ctx {
    pub(super) async fn new(db: &Db) -> Self {
        Self {
            topic: Topic::new(db).await,
            question: Questions::default(),
            popup: Default::default(),
            select_popup: Default::default(),
        }
    }
}
