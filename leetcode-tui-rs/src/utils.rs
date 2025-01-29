use api::{GQLLeetcodeRequest, QuestionRequest};
use color_eyre::Result;
use kdam::BarExt;
use leetcode_core as api;
use leetcode_tui_core::emit;
use leetcode_tui_db::DbQuestion;

fn should_update_db(runs_inside_tui: bool) -> Result<bool> {
    if runs_inside_tui {
        return Ok(true);
    }

    if DbQuestion::get_total_questions()? == 0 || !runs_inside_tui {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub async fn update_database_questions(runs_inside_tui: bool) -> Result<()> {
    if !should_update_db(runs_inside_tui)? {
        return Ok(());
    }

    let query = api::QuestionRequest::default();
    let query_response = query.send().await?;
    let total_questions = query_response.get_total_questions();

    let chunk_size = 1000;
    let total_pages = (total_questions + chunk_size - 1) / chunk_size;

    let mut handles = vec![];

    for page in 0..(total_pages) {
        let skip = page * chunk_size;
        let take = chunk_size.min(total_questions - skip);

        // Here you would typically use .skip(skip).take(take) on your data source
        let join_handle = tokio::spawn(async move {
            let resp = QuestionRequest::new(take, skip).send().await.unwrap();
            let questions = resp.get_questions();
            let db_questions = questions
                .into_iter()
                .map(|q| q.try_into().unwrap())
                .collect::<Vec<DbQuestion>>();

            db_questions
        });

        handles.push(join_handle);
    }

    let mut cli_progress_bar = kdam::tqdm!(total = total_questions as usize);
    let mut all_questions = vec![];
    for handle in handles {
        let questions_result = handle.await.unwrap();
        all_questions.extend(questions_result);

        // update progress bar
        if runs_inside_tui {
            // tui progress bar
            let inside_tui_progress_bar_title = "Syncing db...".into();
            emit!(ProgressUpdate(
                inside_tui_progress_bar_title,
                all_questions.len() as u32,
                total_questions as u32
            ))
        } else {
            // kdam
            cli_progress_bar.update(chunk_size as usize).unwrap();
        }
    }

    DbQuestion::save_multiple_to_db(all_questions);
    Ok(())
}
