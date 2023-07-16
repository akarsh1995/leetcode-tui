use crate::{entities::question::Model as QuestionModel, deserializers::question::Question};
use crate::entities::topic_tag::Model as TopicTagModel;
use std::{collections::HashMap, error};

use super::list::StatefulList;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub type SS = (TopicTagModel, Vec<QuestionModel>);

pub type TTReciever = tokio::sync::mpsc::Receiver<SS>;
pub type TTSender = tokio::sync::mpsc::Sender<SS>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub topic_tags_stateful: StatefulList<TopicTagModel>,

    // multi select logic can be implemented
    pub questions_stateful: StatefulList<QuestionModel>,

    pub questions_list: HashMap<String, Vec<QuestionModel>>,

    pub questions_recv: TTReciever,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(rec: TTReciever) -> Self {
        Self {
            running: true,
            topic_tags_stateful: StatefulList::with_items(vec![]),
            questions_stateful: StatefulList::with_items(vec![]),
            questions_list: HashMap::new(),
            questions_recv: rec,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        for _ in 0..100 {
            if let Some((topic_tag, mut questions)) = self.questions_recv.blocking_recv() {
                if let Some(name) = &topic_tag.name {
                    self.questions_list.entry(name.clone()).or_insert(vec![]).append(&mut questions);
                }
                self.topic_tags_stateful.add_item(topic_tag);
            }
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

}
