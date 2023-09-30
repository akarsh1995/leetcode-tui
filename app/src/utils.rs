use api::{Client, GQLLeetcodeRequest, QuestionRequest};
use color_eyre::Result;
use kdam::BarExt;
use leetcode_core as api;
use leetcode_db::{Db, DbQuestion};

pub async fn update_database_questions(client: &Client, database_client: &Db) -> Result<()> {
    let mut db_question_count = 0;

    if let Ok(c) = DbQuestion::get_total_questions(database_client).await {
        db_question_count = c.count as i32;
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
        let resp = QuestionRequest::new(chunk_size, skip).send(&client).await?;
        let questions = resp.get_questions();
        if questions.is_empty() {
            break;
        }
        let db_questions = questions
            .into_iter()
            .map(|q| q.try_into().unwrap())
            .collect::<Vec<DbQuestion>>();
        for question in db_questions {
            question.to_db(database_client).await?;
            pb.update(1)?;
        }
        skip += chunk_size;
    }
    eprintln!();

    Ok(())
}
