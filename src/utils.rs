use crate::deserializers::problemset_question_list::Question;
use crate::errors::AppResult;
use crate::graphql::problemset_question_list::Query as QuestionDbQuery;
use crate::graphql::GQLLeetcodeQuery;
use crate::{config::Config, db_ops::ModelUtils};
use sea_orm::DatabaseConnection;

pub async fn update_database_questions(
    client: &reqwest::Client,
    database_client: &DatabaseConnection,
) -> AppResult<()> {
    let query = QuestionDbQuery::default();
    let query_response = query.post(client).await?;
    let total_questions = query_response.get_total_questions();

    let chunk_size = 100;
    let n_chunks = total_questions / chunk_size;
    for i in kdam::tqdm!(0..n_chunks) {
        let skip = i * chunk_size;
        let take = chunk_size;
        let client_copy = client.clone();
        let db_client_copy = database_client.clone();
        let resp = QuestionDbQuery::new(take, skip).post(&client_copy).await?;
        let questions = resp.get_questions();
        Question::multi_insert(&db_client_copy, questions).await?;
    }

    if total_questions % chunk_size != 0 {
        let skip = n_chunks * chunk_size;
        let take = total_questions - skip;
        let client_copy = client.clone();
        let db_client_copy = database_client.clone();
        let resp = QuestionDbQuery::new(take, skip).post(&client_copy).await?;
        Question::multi_insert(&db_client_copy, resp.get_questions()).await?;
    }
    Ok(())
}

use crate::migrations::{Migrator, MigratorTrait};

pub async fn do_migrations(database_client: &DatabaseConnection) -> AppResult<()> {
    Ok(Migrator::up(database_client, None).await?)
}

use reqwest::header::{HeaderMap, HeaderValue};

pub async fn get_reqwest_client(config: &Config) -> AppResult<reqwest::Client> {
    let csrf = config.leetcode.csrftoken.as_str();
    let sess = config.leetcode.leetcode_session.as_str();
    let mut headers = HeaderMap::new();
    let header_k_v = [
        (
            "Cookie",
            format!("LEETCODE_SESSION={sess}; csrftoken={csrf}"),
        ),
        ("Content-Type", "application/json".to_string()),
        ("x-csrftoken", csrf.to_string()),
        ("Origin", "https://leetcode.com".to_string()),
        ("Referer", "https://leetcode.com".to_string()),
        ("Connection", "keep-alive".to_string()),
    ];

    for (key, value) in header_k_v {
        headers.append(key, HeaderValue::from_str(value.as_str())?);
    }

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

use crate::config::Db;

pub async fn get_config() -> AppResult<Option<Config>> {
    let config_path = Config::get_config_base_file()?;
    let config: Config;

    if !config_path.exists() {
        config = Config::default();
        config.write_config(config_path.clone()).await?;
        println!("\nConfig is created at config_path {}.\n\nKindly set LEETCODE_SESSION and csrftoken in the config file. These can be obained from leetcode cookies in the browser.", config_path.display());
        let db_data_path = Db::get_base_sqlite_data_path()?;
        if !db_data_path.exists() {
            Db::touch_default_db().await?;
            println!("\nDatabase resides in {}", db_data_path.display());
        }
        if !Config::get_default_solutions_dir()?.exists() {
            Config::create_solutions_dir().await?;
        }
        Ok(None)
    } else {
        println!("Config file found @ {}", &config_path.display());
        config = Config::read_config(config_path).await?;
        Ok(Some(config))
    }
}

use crate::app_ui::async_task_channel::{ChannelRequestReceiver, ChannelResponseSender};

pub async fn async_tasks_executor(
    mut rx_request: ChannelRequestReceiver,
    tx_response: ChannelResponseSender,
    client: &reqwest::Client,
    conn: &DatabaseConnection,
) -> AppResult<()> {
    while let Some(task) = rx_request.recv().await {
        let response = task.execute(client, conn).await;
        tx_response.send(response).map_err(Box::new)?;
    }
    Ok(())
}
