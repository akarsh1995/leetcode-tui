use crate::emit;

use super::utils::Paginate;
use leetcode_db::{Db, DbTopic};
use std::default::Default;

pub struct Topic {
    pub visible: bool,
    pub paginate: Paginate<DbTopic>,
}

impl Topic {
    pub async fn new(db: &Db) -> Self {
        let topics = DbTopic::fetch_all(db).await.unwrap();
        Self {
            paginate: Paginate::new(topics),
            visible: Default::default(),
        }
    }

    pub fn next(&mut self) -> bool {
        self.paginate.next()
    }

    pub fn prev(&mut self) -> bool {
        self.paginate.prev()
    }

    pub fn window(&self) -> &[DbTopic] {
        self.paginate.window()
    }

    pub fn set_visible(&mut self) {
        self.visible = true;
    }
}

impl Topic {
    pub fn hovered(&self) -> Option<&DbTopic> {
        self.paginate.hovered()
    }
}
