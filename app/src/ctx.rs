use app_core::input::Input;
use app_core::popup::{Popup, SelectPopup};
use app_core::question::Questions;
use app_core::topic::Topic;
use leetcode_db::Db;

pub struct Ctx {
    pub topic: Topic,
    pub question: Questions,
    pub popup: Popup,
    pub select_popup: SelectPopup<String>,
    pub input: Input,
}

impl Ctx {
    pub(super) async fn new(db: &Db) -> Self {
        Self {
            topic: Topic::new(db).await,
            question: Questions::default(),
            popup: Default::default(),
            select_popup: Default::default(),
            input: Default::default(),
        }
    }
}
