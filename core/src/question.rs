use config::log;
use config::DB_CLIENT;
use config::REQ_CLIENT;
use leetcode_core::{GQLLeetcodeRequest, QuestionContentRequest};
use leetcode_db::{DbQuestion, DbTopic};

use crate::{emit, utils::Paginate};

pub struct Questions {
    paginate: Paginate<DbQuestion>,
}

impl Default for Questions {
    fn default() -> Self {
        Self {
            paginate: Paginate::new(vec![]),
        }
    }
}

impl Questions {
    pub fn prev_ques(&mut self) -> bool {
        self.paginate.prev_elem()
    }

    pub fn next_ques(&mut self) -> bool {
        self.paginate.next_elem()
    }

    pub fn window(&self) -> &[DbQuestion] {
        self.paginate.window()
    }

    pub fn hovered(&self) -> Option<&DbQuestion> {
        self.paginate.hovered()
    }

    pub fn show_question_content(&self) -> bool {
        if let Some(_hovered) = self.hovered() {
            let slug = _hovered.title_slug.clone();
            tokio::spawn(async move {
                let qc = QuestionContentRequest::new(slug);
                let content = qc.send(REQ_CLIENT.as_ref()).await;
                match content {
                    Ok(c) => {
                        let lines = c
                            .data
                            .question
                            .html_to_text()
                            .lines()
                            .map(|l| l.to_string())
                            .collect::<Vec<String>>();
                        emit!(Popup(lines));
                    }
                    Err(e) => {
                        emit!(Error(e.to_string()));
                    }
                };
                emit!(Render);
            });
        } else {
            log::debug!("hovered question is none");
        }
        false
    }
}

impl Questions {
    pub fn get_questions_by_topic(&mut self, topic: DbTopic) {
        tokio::spawn(async move {
            let questions = topic.fetch_questions(DB_CLIENT.as_ref()).await;
            match questions {
                Ok(_questions) => {
                    emit!(Questions(_questions));
                }
                Err(e) => {
                    log::error!("Problem fetching questions for topic {topic:?}: {e}");
                    emit!(Error(e.to_string()));
                }
            }
            emit!(Render);
        });
    }

    pub fn set_questions(&mut self, questions: Vec<DbQuestion>) {
        self.paginate.update_list(questions)
    }
}
