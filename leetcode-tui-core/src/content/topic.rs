use crate::emit;
use crate::utils::Paginate;
use leetcode_tui_db::DbTopic;
use leetcode_tui_shared::layout::Window;

pub struct Topic {
    paginate: Paginate<DbTopic>,
    topics: Vec<DbTopic>,
}

impl<'a> Topic {
    pub(crate) async fn new() -> Self {
        let mut topics = vec![DbTopic::new("all")];
        topics.extend(DbTopic::fetch_all().unwrap());
        let s = Self {
            paginate: Paginate::new(topics.clone()),
            topics,
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

    pub fn notify_change(&self) {
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

    pub fn set_topic(&mut self, topic: &DbTopic) -> bool {
        if let Some(id) = self.topics.iter().position(|x| x.slug == topic.slug) {
            self.paginate.set_element_by_index(id, self.widget_height());
        }
        return true;
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
