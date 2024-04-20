use api::{Client, GQLLeetcodeRequest, QuestionRequest};
use color_eyre::Result;
use config::clients::Db;
use kdam::BarExt;
use leetcode_core as api;
use leetcode_db::DbQuestion;

pub async fn update_database_questions() -> Result<()> {
    let client: &Client = config::REQ_CLIENT.as_ref();
    let database_client: &Db = config::DB_CLIENT.as_ref();
    let mut db_question_count = 0;

    if let Ok(c) = DbQuestion::get_total_questions(database_client) {
        db_question_count = c as i32;
    }

    let query = api::QuestionRequest::default();
    let query_response = query.send(client).await?;
    let total_questions = query_response.get_total_questions();

    if db_question_count == total_questions {
        return Ok(());
    }

    println!(
        "Questions found in db: {}\nQuestions found in api: {}, Updating",
        db_question_count, total_questions
    );

    let mut skip = 0;
    let chunk_size = 100;
    let mut pb = kdam::tqdm!(total = total_questions as usize);

    loop {
        let resp = QuestionRequest::new(chunk_size, skip).send(client).await?;
        let questions = resp.get_questions();
        if questions.is_empty() {
            break;
        }
        let mut db_questions = questions
            .into_iter()
            .map(|q| q.try_into().unwrap())
            .collect::<Vec<DbQuestion>>();
        for question in db_questions.iter_mut() {
            question.save_to_db(database_client)?;
            pb.update(1)?;
        }
        skip += chunk_size;
    }
    eprintln!();

    Ok(())
}
