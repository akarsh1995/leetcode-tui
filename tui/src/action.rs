use std::sync::Arc;

use leetcode_db::DbQuestion;

use crate::key::Key;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Error(String),
    Quit,
    Tick,
    Init,
    Render,
    Resume,
    Suspend,
    Resize(u16, u16),
    NextTopic,
    PreviousTopic,
    NextQuestion,
    PreviousQuestion,
    UpdateQuestions(Arc<Vec<DbQuestion>>),
    SetQuestions,
    SetHelpBar(Vec<(Vec<Key>, String)>),
}
