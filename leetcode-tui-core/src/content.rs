pub mod question;
use leetcode_tui_config::DB_CLIENT;
use question::Questions;
use topic::Topic;
pub mod topic;

pub mod questions {}

pub struct MainContent {
    topic: Topic,
    questions: Questions,
    visible: bool,
}

impl MainContent {
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl MainContent {
    pub async fn new() -> Self {
        Self {
            topic: Topic::new(DB_CLIENT.as_ref()).await,
            questions: Default::default(),
            visible: true,
        }
    }

    pub fn get_topic_mut(&mut self) -> &mut Topic {
        &mut self.topic
    }

    pub fn get_topic(&self) -> &Topic {
        &self.topic
    }

    pub fn get_questions_mut(&mut self) -> &mut Questions {
        &mut self.questions
    }

    pub fn get_questions(&self) -> &Questions {
        &self.questions
    }
}
