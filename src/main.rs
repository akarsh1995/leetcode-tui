use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use kdam;

use leetcode_tui_rs::migrations::{Migrator, MigratorTrait};

use leetcode_tui_rs::app_ui::channel::{self, request_channel, response_channel};
use leetcode_tui_rs::app_ui::channel::{
    ChannelRequestSender, ChannelResponseReceiver, Request, Response,
};
use leetcode_tui_rs::app_ui::list::StatefulList;
use leetcode_tui_rs::app_ui::tui::Tui;
use leetcode_tui_rs::app_ui::ui::render;
use leetcode_tui_rs::config::{self, Config, Db};
use leetcode_tui_rs::db_ops::ModelUtils;
use leetcode_tui_rs::deserializers;
use leetcode_tui_rs::deserializers::question::Question;
use leetcode_tui_rs::deserializers::question_content::{QueryQuestionContent, QuestionContent};
use leetcode_tui_rs::entities::QuestionModel;
use leetcode_tui_rs::errors::AppResult;
use leetcode_tui_rs::graphql::problemset_question_list::Query;
use leetcode_tui_rs::graphql::{question_content, GQLLeetcodeQuery};
use reqwest::header::{HeaderMap, HeaderValue};
use sea_orm::{ColIdx, Database};
use tokio::task::JoinHandle;
use tracing;
use tracing_subscriber;

use leetcode_tui_rs::app_ui::app::{App, TTReciever, Widget};
use leetcode_tui_rs::app_ui::event::{look_for_events, Event, EventHandler};
use leetcode_tui_rs::app_ui::handler::handle_key_events;
// use leetcode_tui_rs::app_ui::tui::Tui;
use leetcode_tui_rs::entities::topic_tag::Model as TopicTagModel;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::collections::HashMap;
use std::io::{self, Stderr, Stdout};

use once_cell::sync::Lazy;

// static CONFIG: Lazy<config::Config> = Lazy::new(|| ));

use xdg;

#[tokio::main]
async fn main() -> AppResult<()> {
    // let config = Config::read_config("./leetcode.config").await?;
    let config_path = Config::get_base_config()?;
    let config: Config;

    if !config_path.exists() {
        config = Config::default();
        config.write_config(Config::get_base_config()?).await?;
        println!("\nConfig is created at config_path {}.\n Kindly set LEETCODE_SESSION and csrftoken in the config file. These can be obained from leetcode cookies in the browser.", config_path.display());
        let db_data_path = Db::get_base_sqlite_data_path()?;
        if !db_data_path.exists() {
            Db::touch_default_db().await?;
            println!("\nDatabase resides in {}", db_data_path.display());
        }
        return Ok(());
    } else {
        println!("Config file found @ {}", &config_path.display());
        config = Config::read_config(config_path).await?;
    }

    let client: reqwest::Client = {
        let csrf = config.leetcode.csrftoken.as_str();
        let sess = config.leetcode.leetcode_session.as_str();
        let mut headers = HeaderMap::new();
        headers.append(
            "Cookie",
            HeaderValue::from_str(&format!("LEETCODE_SESSION={sess}; csrftoken={csrf}")).unwrap(),
        );

        headers.append(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        headers.append(
            "x-csrftoken",
            HeaderValue::from_str(csrf.as_str().unwrap()).unwrap(),
        );

        headers.append(
            "Origin",
            HeaderValue::from_str("https://leetcode.com").unwrap(),
        );

        headers.append(
            "Referer",
            HeaderValue::from_str("https://leetcode.com").unwrap(),
        );

        headers.append("Connection", HeaderValue::from_str("keep-alive").unwrap());

        reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap()
    };

    let database_client = Database::connect(format!("{}?mode=rwc", config.db.url.as_str())).await?;

    Migrator::up(&database_client, None).await?;

    let query = leetcode_tui_rs::graphql::problemset_question_list::Query::default();
    let query_response: deserializers::question::ProblemSetQuestionListQuery =
        query.post(&client).await?;
    let total_questions = query_response.get_total_questions();
    println!("Requesting {} questions from leetcode.", total_questions);

    let chunk_size = 100;
    let n_chunks = total_questions / chunk_size;
    for i in kdam::tqdm!(26..n_chunks) {
        let skip = i * chunk_size;
        let take = chunk_size;
        let client_copy = client.clone();
        let db_client_copy = database_client.clone();
        println!("{}, {}", &skip, &take);

        // handle when topic tags are empty
        let resp = Query::new(take, skip).post(&client_copy).await?;
        let questions = resp
            .get_questions()
            .into_iter()
            .map(|x| x)
            .filter(|q| !q.topic_tags.as_ref().unwrap().is_empty())
            .collect::<Vec<_>>();

        Question::multi_insert(&db_client_copy, questions).await?;
    }

    if total_questions % chunk_size != 0 {
        let skip = n_chunks * chunk_size;
        let take = total_questions - skip;
        let client_copy = client.clone();
        let db_client_copy = database_client.clone();
        let resp = Query::new(take, skip).post(&client_copy).await?;
        Question::multi_insert(&db_client_copy, resp.get_questions()).await?;
    }

    // if total_questions % 20 == 0 {}

    // Question::multi_insert(&database_client, query_response.get_questions()).await;

    // // Create an application.
    // use crossbeam;

    // let (send, recv) = crossbeam::channel::unbounded();

    // let mut q =
    //     leetcode_tui_rs::db_ops::topic_tag::query::get_questions_by_topic(&database_client, "")
    //         .await;

    // while !q.is_empty() {
    //     let qp = q.pop();
    //     if let Some(qp) = qp {
    //         send.send(qp).unwrap();
    //     };
    // }

    // drop(send);

    // let (tx_request, rx_request) = request_channel();
    // let (tx_response, rx_response) = response_channel();
    // let client = client.clone();

    // let jh: JoinHandle<AppResult<()>> = tokio::spawn(async move {
    //     let mut rx_request = rx_request;
    //     let tx_response = tx_response;
    //     while let Some(task) = rx_request.recv().await {
    //         match task {
    //             Request::QuestionDetail { slug } => {
    //                 match question_content::Query::new(slug).post(&client).await {
    //                     Ok(resp) => {
    //                         let query_response: deserializers::question_content::Data = resp;
    //                         tx_response.send(channel::Response::QuestionDetail(
    //                             query_response.data.question,
    //                         ))?;
    //                     }
    //                     Err(e) => tx_response.send(channel::Response::Error(e.to_string()))?,
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // });

    // let backend = CrosstermBackend::new(io::stderr());
    // let terminal = Terminal::new(backend)?;

    // let (ev_sender, ev_receiver) = std::sync::mpsc::channel();

    // let mut tui = Tui::new(
    //     terminal,
    //     EventHandler {
    //         sender: ev_sender.clone(),
    //         receiver: ev_receiver,
    //     },
    // );

    // tui.init()?;

    // tokio::task::spawn_blocking(move || run_app(recv, tx_request, rx_response, tui).unwrap());

    // look_for_events(100, ev_sender).await?;

    Ok(())
}

fn run_app(
    recv: TTReciever,
    tx_request: ChannelRequestSender,
    rx_response: ChannelResponseReceiver,
    mut tui: Tui<CrosstermBackend<Stderr>>,
) -> AppResult<()> {
    let mut ql: HashMap<String, Vec<QuestionModel>> = HashMap::new();
    let mut topic_tags = vec![];

    topic_tags.push(TopicTagModel {
        name: Some("All".to_string()),
        id: "all".to_string(),
        slug: Some("all".to_string()),
    });

    while let Ok((topic_tag, mut questions)) = recv.recv() {
        if let Some(name) = &topic_tag.name {
            ql.entry(name.clone())
                .or_insert(vec![])
                .append(&mut questions);
        }
        topic_tags.push(topic_tag);
    }

    let questions = vec![];

    let mut qm: StatefulList<QuestionModel> = StatefulList::with_items(questions);
    let mut ttm: StatefulList<TopicTagModel> = StatefulList::with_items(topic_tags);
    ttm.state.select(Some(0));
    let question_stateful = Widget::QuestionList(&mut qm);
    let topic_tag_stateful = Widget::TopicTagList(&mut ttm);
    let mut vw = vec![topic_tag_stateful, question_stateful];

    let mut app = App::new(&mut vw, &ql, tx_request, rx_response);

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
