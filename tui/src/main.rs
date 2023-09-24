use color_eyre::Result;
use leetcode_core::{build_reqwest_client, Client, GQLLeetcodeRequest, QuestionRequest};
use leetcode_db::{connect, Db, DbQuestion};
use tui::runner::Runner;

pub async fn update_database_questions(client: &Client, database_client: &Db) -> Result<()> {
    let query = QuestionRequest::default();
    let query_response = query.send(client).await?;
    let total_questions = query_response.get_total_questions();
    let chunk_size: usize = 100;
    let total_range = (0..total_questions).collect::<Vec<_>>();
    for chunk in kdam::tqdm!(total_range.chunks(chunk_size)) {
        if let Some(skip) = chunk.first() {
            let resp = QuestionRequest::new(chunk.len() as i32, *skip)
                .send(&client)
                .await?;
            let questions = resp.get_questions();
            let db_questions = questions
                .into_iter()
                .map(|q| q.try_into().unwrap())
                .collect::<Vec<DbQuestion>>();
            for question in db_questions {
                question.to_db(database_client).await.unwrap();
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let csrf = "q8VywCStpcPGivpOD3k6fAOzi96YkbtegaT8zAzQxSQJtENRuvl0uraafBGFIAml";
    let sess="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJfYXV0aF91c2VyX2lkIjoiNTA1MTA2NyIsIl9hdXRoX3VzZXJfYmFja2VuZCI6ImFsbGF1dGguYWNjb3VudC5hdXRoX2JhY2tlbmRzLkF1dGhlbnRpY2F0aW9uQmFja2VuZCIsIl9hdXRoX3VzZXJfaGFzaCI6ImEyMzY4MWU3OWI3MzRhMDY4ZGQxNzFlZjQ4OTAzYjhlZjhkN2ViOGQiLCJpZCI6NTA1MTA2NywiZW1haWwiOiJha2Fyc2guMTk5NS4wMkBnbWFpbC5jb20iLCJ1c2VybmFtZSI6InVzZXI4MTYybCIsInVzZXJfc2x1ZyI6InVzZXI4MTYybCIsImF2YXRhciI6Imh0dHBzOi8vYXNzZXRzLmxlZXRjb2RlLmNvbS91c2Vycy91c2VyODE2MmwvYXZhdGFyXzE2MzM3NzQzODAucG5nIiwicmVmcmVzaGVkX2F0IjoxNjk1NDAwMjg0LCJpcCI6IjEyMi4xNzIuMjQzLjIwNiIsImlkZW50aXR5IjoiYjkyYjE1YWM0MmZjM2U3NTc1NWQ2ODViMjkwMGRhMTkiLCJzZXNzaW9uX2lkIjo0Mjg0MzUzNn0.ijIjD0GVKFXaHWC2jBBlCcO_cN-a3Uw-MaNZYmnqFV8";
    let client = build_reqwest_client(csrf, sess).await.unwrap();
    let connection = connect("mem://").await.unwrap();
    connection.use_ns("test").use_db("test").await.unwrap();
    update_database_questions(&client, &connection)
        .await
        .unwrap();
    let mut runner = Runner::new(0.2, 0.2, &connection).await.unwrap();
    runner.run().await.unwrap();

    Ok(())
}
