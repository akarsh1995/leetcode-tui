pub mod console_panel_config;
pub mod editor_data;
pub mod problemset_question_list;
pub mod question_content;
pub mod run_code;
pub mod submit_code;
use super::GQLLeetcodeRequest;
use crate::errors::AppResult;
use crate::types::run_submit_response::{ParsedResponse, RunSubmitResult};
use async_trait::async_trait;
pub use editor_data::Query as EditorDataRequest;

#[async_trait]
trait RunOrSubmitCodeCheckResult<IntermediateResponse>:
    GQLLeetcodeRequest<T = IntermediateResponse>
where
    IntermediateResponse: GQLLeetcodeRequest<T = RunSubmitResult> + Send,
{
    async fn poll_check_response(&self, client: &reqwest::Client) -> AppResult<ParsedResponse> {
        let run_response = self.send(client).await?;
        loop {
            let status_check = run_response.send(client).await?;
            let parsed_response = status_check.to_parsed_response()?;
            match parsed_response {
                ParsedResponse::Pending => continue,
                _ => return Ok(parsed_response),
            }
        }
    }
}
