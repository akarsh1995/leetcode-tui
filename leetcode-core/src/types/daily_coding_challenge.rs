use super::problemset_question_list::Question;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub date: String,
    pub user_status: String,
    pub link: String,
    pub question: Question,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDailyCodingChallengeQuestion {
    pub active_daily_coding_challenge_question: Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IDailyCodingChallenge {
    pub data: ActiveDailyCodingChallengeQuestion,
}
