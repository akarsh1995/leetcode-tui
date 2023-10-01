pub mod errors;
pub mod graphql;
pub mod types;

use errors::AppResult;
pub use graphql::client::GQLLeetcodeRequest;
pub use graphql::query::problemset_question_list::Query as QuestionRequest;
pub use reqwest::Client;
pub use types::problemset_question_list::Root as QuestionResponse;

pub use graphql::query::question_content::Query as QuestionContentRequest;
