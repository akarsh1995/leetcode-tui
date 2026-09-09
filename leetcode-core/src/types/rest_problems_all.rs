use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Stat {
    pub question_id: u32,
    #[serde(rename = "question__title")]
    pub title: String,
    #[serde(rename = "question__title_slug")]
    pub title_slug: String,
    #[serde(rename = "question__hide")]
    pub hide: bool,
    #[serde(rename = "total_acs")]
    pub total_acs: u64,
    #[serde(rename = "total_submitted")]
    pub total_submitted: u64,
    #[serde(rename = "frontend_question_id")]
    pub frontend_question_id: u32,
    #[serde(rename = "is_new_question")]
    pub is_new_question: bool,
}

#[derive(Debug, Deserialize)]
pub struct Difficulty {
    pub level: u32,
}

#[derive(Debug, Deserialize)]
pub struct Problem {
    pub stat: Stat,
    pub status: Option<String>,
    pub difficulty: Difficulty,
    #[serde(rename = "paid_only")]
    pub paid_only: bool,
    #[serde(rename = "is_favor")]
    pub is_favor: bool,
    pub frequency: f64,
    pub progress: f64,
}

#[derive(Debug, Deserialize)]
pub struct ProblemsAllResponse {
    pub user_name: String,
    pub num_solved: u32,
    pub num_total: u32,
    pub ac_easy: u32,
    pub ac_medium: u32,
    pub ac_hard: u32,
    pub category_slug: String,
    pub problems: Vec<Problem>,
}

impl Problem {
    pub fn difficulty_str(&self) -> String {
        match self.difficulty.level {
            1 => "Easy".to_string(),
            2 => "Medium".to_string(),
            3 => "Hard".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
