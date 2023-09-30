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
        let s = Self {
            paginate: Paginate::new(topics),
            visible: Default::default(),
        };
        s.notify_change();
        s
    }

    pub fn next(&mut self) -> bool {
        let n = self.paginate.next();
        self.notify_change();
        n
    }

    fn notify_change(&self) {
        if let Some(hovered) = self.hovered() {
            emit!(Topic(hovered.clone()));
        }
    }

    pub fn prev(&mut self) -> bool {
        let n = self.paginate.prev();
        self.notify_change();
        n
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
