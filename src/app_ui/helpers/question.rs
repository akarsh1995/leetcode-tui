use crate::entities::QuestionModel;

pub struct Stats<'a> {
    pub qm: &'a Vec<QuestionModel>,
}

impl<'a> Stats<'a> {
    pub(crate) fn get_total_question(&self) -> usize {
        self.qm.len()
    }

    pub(crate) fn get_locked(&self) -> usize {
        self.qm.iter().filter(|q| q.paid_only.unwrap() == 1).count()
    }

    pub(crate) fn get_starred(&self) -> usize {
        self.qm
            .iter()
            .filter(|q| {
                if let Some(starred) = &q.is_favor {
                    starred == &1
                } else {
                    false
                }
            })
            .count()
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
                if let Some(st) = &q.status {
                    if let Some(at) = &q.difficulty {
                        st.as_str() == status && difficulty == at.as_str()
                    } else {
                        false
                    }
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
                if let Some(st) = &q.status {
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
            .filter(|q| {
                if let Some(diff) = &q.difficulty {
                    diff.as_str() == difficulty
                } else {
                    false
                }
            })
            .count()
    }
}
