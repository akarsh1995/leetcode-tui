pub mod errors;
pub mod graphql;
pub mod types;

pub use graphql::client::GQLLeetcodeRequest;
pub use graphql::query::problemset_question_list::Query as QuestionRequest;
pub use graphql::query::EditorDataRequest;
pub use reqwest::Client;
pub use types::editor_data::QuestionData as EditorDataResponse;
pub use types::problemset_question_list::Root as QuestionResponse;

pub use graphql::query::question_content::Query as QuestionContentRequest;
