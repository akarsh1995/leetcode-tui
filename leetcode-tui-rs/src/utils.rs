use api::{GQLLeetcodeRequest, QuestionRequest};
use color_eyre::Result;
use kdam::BarExt;
use leetcode_core as api;
use leetcode_tui_core::emit;
use leetcode_tui_db::DbQuestion;
use std::time::Duration;

fn should_update_db(runs_inside_tui: bool) -> Result<bool> {
    let first_time_start = DbQuestion::get_total_questions()? == 0 && !runs_inside_tui;
    if first_time_start || runs_inside_tui {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

async fn sync_via_rest_api(runs_inside_tui: bool) -> Result<Vec<DbQuestion>> {
    log::info!("Attempting to sync via REST API /api/problems/all");
    let response = api::fetch_all_problems_rest().await?;

    let total = response.problems.len();
    log::info!("REST API returned {} problems", total);

    let mut db_questions = Vec::with_capacity(total);
    let mut skipped = 0;

    for problem in &response.problems {
        if problem.stat.hide {
            skipped += 1;
            continue;
        }

        match DbQuestion::try_from_rest_problem(problem) {
            Ok(q) => db_questions.push(q),
            Err(e) => {
                log::warn!(
                    "Failed to convert problem {} ({}): {}",
                    problem.stat.frontend_question_id,
                    problem.stat.title,
                    e
                );
            }
        }
    }

    if skipped > 0 {
        log::info!("Skipped {} hidden problems", skipped);
    }

    if runs_inside_tui {
        emit!(ProgressUpdate(
            "Syncing db...".into(),
            db_questions.len() as u32,
            db_questions.len() as u32
        ));
    }

    Ok(db_questions)
}

async fn sync_via_graphql(runs_inside_tui: bool) -> Result<Vec<DbQuestion>> {
    log::info!("Falling back to GraphQL API for sync");
    let query = api::QuestionRequest::default();
    let query_response = query.send().await?;
    let total_questions = query_response.get_total_questions();

    let chunk_size = 50;
    let total_pages = (total_questions + chunk_size - 1) / chunk_size;

    let mut all_questions = Vec::with_capacity(total_questions as usize);
    let mut failed_pages = 0;

    let mut cli_progress_bar = kdam::tqdm!(total = total_questions as usize);

    for page in 0..total_pages {
        let skip = page * chunk_size;
        let take = chunk_size.min(total_questions - skip);

        match QuestionRequest::new(take, skip).send().await {
            Ok(resp) => {
                let questions = resp.get_questions();
                let db_questions: Vec<DbQuestion> = questions
                    .into_iter()
                    .filter_map(|q| match q.try_into() {
                        Ok(q) => Some(q),
                        Err(e) => {
                            log::warn!("Failed to convert question: {}", e);
                            None
                        }
                    })
                    .collect();

                all_questions.extend(db_questions);
            }
            Err(e) => {
                log::warn!("Failed to fetch page {} (skip={}): {}", page, skip, e);
                failed_pages += 1;
            }
        }

        if runs_inside_tui {
            emit!(ProgressUpdate(
                "Syncing db...".into(),
                all_questions.len() as u32,
                total_questions as u32
            ));
        } else {
            cli_progress_bar.update(take as usize)?;
        }

        if page < total_pages - 1 {
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }

    if failed_pages > 0 {
        log::warn!("{} pages failed to fetch", failed_pages);
    }

    Ok(all_questions)
}

pub async fn update_database_questions(runs_inside_tui: bool) -> Result<()> {
    if !should_update_db(runs_inside_tui)? {
        return Ok(());
    }

    let all_questions = match sync_via_rest_api(runs_inside_tui).await {
        Ok(questions) => {
            log::info!(
                "Successfully synced {} questions via REST API",
                questions.len()
            );
            questions
        }
        Err(e) => {
            log::warn!("REST API sync failed: {}. Falling back to GraphQL.", e);
            sync_via_graphql(runs_inside_tui).await?
        }
    };

    if all_questions.is_empty() {
        log::warn!("No questions were fetched. Database will not be updated.");
        return Ok(());
    }

    DbQuestion::save_multiple_to_db(all_questions);
    Ok(())
}
