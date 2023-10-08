use crate::emit;
use crate::utils::Paginate;
use leetcode_db::{Db, DbTopic};
use shared::layout::Window;

pub struct Topic {
    paginate: Paginate<DbTopic>,
}

impl Topic {
    pub(crate) async fn new(db: &Db) -> Self {
        let topics = DbTopic::fetch_all(db).await.unwrap();
        let s = Self {
            paginate: Paginate::new(topics),
        };
        s.notify_change();
        s
    }

    pub fn next_topic(&mut self) -> bool {
        let has_topic_changed = self.paginate.next_elem(self.widget_height());
        if has_topic_changed {
            self.notify_change();
        }
        has_topic_changed
    }

    fn notify_change(&self) {
        if let Some(hovered) = self.hovered() {
            emit!(Topic(hovered.clone()));
        }
    }

    pub fn prev_topic(&mut self) -> bool {
        let has_topic_changed = self.paginate.prev_elem(self.widget_height());
        if has_topic_changed {
            self.notify_change()
        };
        has_topic_changed
    }

    pub fn window(&self) -> &[DbTopic] {
        self.paginate.window(self.widget_height())
    }

    fn widget_height(&self) -> usize {
        let window = Window::default();
        let height = window.root.center_layout.topic.inner.height;
        height as usize
    }
}

impl Topic {
    pub fn hovered(&self) -> Option<&DbTopic> {
        self.paginate.hovered()
    }
}
