use leetcode_db::{Db, DbQuestion, DbTopic};
use shared::log;

use crate::{emit, utils::Paginate};

pub struct Questions {
    paginate: Paginate<DbQuestion>,
}

impl Questions {
    pub fn new() -> Self {
        Self {
            paginate: Paginate::new(vec![]),
        }
    }
}

impl Questions {
    pub fn prev(&mut self) -> bool {
        self.paginate.prev()
    }

    pub fn next(&mut self) -> bool {
        self.paginate.next()
    }

    pub fn window(&self) -> &[DbQuestion] {
        self.paginate.window()
    }

    pub fn hovered(&self) -> Option<&DbQuestion> {
        self.paginate.hovered()
    }

    pub fn show_question_content(&self) -> bool {
        if let Some(_hovered) = self.hovered() {
            emit!(Popup(vec![_hovered.title_slug.clone()]));
            true
        } else {
            shared::log::debug!("hovered question is none");
            false
        }
    }
}

impl Questions {
    pub fn get_questions_by_topic(&mut self, topic: DbTopic, db: Db) {
        tokio::spawn(async move {
            let questions = topic.fetch_questions(&db.clone()).await;
            match questions {
                Ok(_questions) => {
                    emit!(Questions(_questions));
                    emit!(Render);
                }
                Err(e) => log::error!("Problem fetching questions for topic {topic:?}: {e}"),
            }
        });
    }

    pub fn set_questions(&mut self, questions: Vec<DbQuestion>) {
        self.paginate.update_list(questions)
    }
}
