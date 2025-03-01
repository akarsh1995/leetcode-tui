use leetcode_tui_config::CONFIG;
use leetcode_tui_db::{enums::Difficulty, DbQuestion};
use ratatui::style::Style;
use std::{fmt::Display, rc::Rc};

pub struct Stats<'a> {
    qm: &'a Vec<Rc<DbQuestion>>,
}

impl<'a> Stats<'a> {
    pub(super) fn new(questions: &'a Vec<Rc<DbQuestion>>) -> Self {
        Self { qm: questions }
    }
}

#[derive(Debug)]
pub enum QuestionStatus {
    Accepted,
    Attempted,
    EasyAccepted,
    MediumAccepted,
    HardAccepted,
}

use QuestionStatus::*;

impl Display for QuestionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Accepted => "Accepted",
            Attempted => "Attempted",
            EasyAccepted => "Easy Accepted",
            MediumAccepted => "Medium Accepted",
            HardAccepted => "Hard Accepted",
        };
        write!(f, "{}", string)
    }
}

impl Into<Style> for QuestionStatus {
    fn into(self) -> Style {
        match self {
            Accepted => CONFIG.as_ref().theme.question.normal.easy,
            Attempted => CONFIG.as_ref().theme.question.normal.easy,
            EasyAccepted => CONFIG.as_ref().theme.question.normal.easy,
            MediumAccepted => CONFIG.as_ref().theme.question.normal.medium,
            HardAccepted => CONFIG.as_ref().theme.question.normal.hard,
        }
        .into()
    }
}

impl<'a> Stats<'a> {
    pub fn get_ratios(&self) -> Vec<(QuestionStatus, usize, usize)> {
        use QuestionStatus::*;
        vec![
            (Accepted, self.get_accepted(), self.get_total_question()),
            (
                Attempted,
                self.get_total_question() - self.get_not_attempted(),
                self.get_total_question(),
            ),
            (
                EasyAccepted,
                self.get_easy_accepted(),
                self.get_easy_count(),
            ),
            (
                MediumAccepted,
                self.get_medium_accepted(),
                self.get_medium_count(),
            ),
            (
                HardAccepted,
                self.get_hard_accepted(),
                self.get_hard_count(),
            ),
        ]
    }
}

impl<'a> Stats<'a> {
    pub fn get_total_question(&self) -> usize {
        self.qm.len()
    }

    pub fn get_accepted(&self) -> usize {
        self.get_status("ac")
    }

    pub fn get_not_accepted(&self) -> usize {
        self.get_status("notac")
    }

    pub fn get_not_attempted(&self) -> usize {
        self.get_total_question() - (self.get_accepted() + self.get_not_accepted())
    }

    pub fn get_easy_count(&self) -> usize {
        self.get_diff_count(Difficulty::Easy)
    }

    pub fn get_medium_count(&self) -> usize {
        self.get_diff_count(Difficulty::Medium)
    }

    pub fn get_hard_count(&self) -> usize {
        self.get_diff_count(Difficulty::Hard)
    }

    pub fn get_easy_accepted(&self) -> usize {
        self.get_diff_accepted("ac", Difficulty::Easy)
    }

    pub fn get_medium_accepted(&self) -> usize {
        self.get_diff_accepted("ac", Difficulty::Medium)
    }

    pub fn get_hard_accepted(&self) -> usize {
        self.get_diff_accepted("ac", Difficulty::Hard)
    }

    pub fn get_diff_accepted(&self, status: &str, difficulty: Difficulty) -> usize {
        self.qm
            .iter()
            .filter(|q| {
                if let Some(st) = &q.status {
                    st.as_str() == status && difficulty == q.get_difficulty()
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

    fn get_diff_count(&self, difficulty: Difficulty) -> usize {
        self.qm
            .iter()
            .filter(|q| q.get_difficulty() == difficulty)
            .count()
    }
}
