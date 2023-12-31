use std::{cell::RefCell, rc::Rc};

use crate::entities::QuestionModel;

pub struct Stats<'a> {
    pub qm: &'a Vec<Rc<RefCell<QuestionModel>>>,
}

impl<'a> Stats<'a> {
    pub(crate) fn get_total_question(&self) -> usize {
        self.qm.len()
    }

    pub(crate) fn get_accepted(&self) -> usize {
        self.get_status("ac")
    }

    pub(crate) fn get_not_accepted(&self) -> usize {
        self.get_status("notac")
    }

    pub(crate) fn get_not_attempted(&self) -> usize {
        self.get_total_question() - (self.get_accepted() + self.get_not_accepted())
    }

    pub(crate) fn get_easy_count(&self) -> usize {
        self.get_diff_count("Easy")
    }

    pub(crate) fn get_medium_count(&self) -> usize {
        self.get_diff_count("Medium")
    }

    pub(crate) fn get_hard_count(&self) -> usize {
        self.get_diff_count("Hard")
    }

    pub(crate) fn get_easy_accepted(&self) -> usize {
        self.get_diff_accepted("ac", "Easy")
    }

    pub(crate) fn get_medium_accepted(&self) -> usize {
        self.get_diff_accepted("ac", "Medium")
    }

    pub(crate) fn get_hard_accepted(&self) -> usize {
        self.get_diff_accepted("ac", "Hard")
    }

    pub(crate) fn get_diff_accepted(&self, status: &str, difficulty: &str) -> usize {
        self.qm
            .iter()
            .filter(|q| {
                if let Some(st) = &q.borrow().status {
                    st.as_str() == status && difficulty == q.borrow().difficulty.as_str()
                } else {
                    false
                }
            })
            .count()
    }

    fn get_status(&self, status: &str) -> usize {
        self.qm
            .iter()
            .filter(|q| {
                if let Some(st) = &q.borrow().status {
                    st.as_str() == status
                } else {
                    false
                }
            })
            .count()
    }

    fn get_diff_count(&self, difficulty: &str) -> usize {
        self.qm
            .iter()
            .filter(|q| q.borrow().difficulty.as_str() == difficulty)
            .count()
    }
}
