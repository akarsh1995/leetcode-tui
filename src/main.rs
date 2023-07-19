use crossbeam;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use kdam;
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
use leetcode_tui_rs::utils::{
    do_migrations, get_config, get_reqwest_client, update_database_questions,
};
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
    let config: Config;

    if let Some(c) = get_config().await? {
        config = c;
    } else {
        // files created kindly set leetcode session and csrftoken
        return Ok(());
    }

    let client = get_reqwest_client(&config).await?;
    let database_client = Database::connect(format!("{}?mode=rwc", config.db.url.as_str())).await?;
    let database_client_clone = database_client.clone();
    let client_clone = client.clone();
    do_migrations(&database_client).await?;

    let _async_populate_db: JoinHandle<AppResult<()>> = tokio::spawn(async move {
        update_database_questions(&client, &database_client).await?;
        Ok(())
    });

    let database_client = database_client_clone;
    let client = client_clone;

    let (send, recv) = crossbeam::channel::unbounded();

    let mut q =
        leetcode_tui_rs::db_ops::topic_tag::query::get_questions_by_topic(&database_client, "")
            .await;

    while !q.is_empty() {
        let qp = q.pop();
        if let Some(qp) = qp {
            send.send(qp).unwrap();
        };
    }

    drop(send);

    let (tx_request, rx_request) = request_channel();
    let (tx_response, rx_response) = response_channel();
    let client = client.clone();

    let _task_receiver_from_app: JoinHandle<AppResult<()>> = tokio::spawn(async move {
        let mut rx_request = rx_request;
        let tx_response = tx_response;
        while let Some(task) = rx_request.recv().await {
            match task {
                Request::QuestionDetail { slug } => {
                    match question_content::Query::new(slug).post(&client).await {
                        Ok(resp) => {
                            let query_response: deserializers::question_content::Data = resp;
                            tx_response.send(channel::Response::QuestionDetail(
                                query_response.data.question,
                            ))?;
                        }
                        Err(e) => tx_response.send(channel::Response::Error(e.to_string()))?,
                    }
                }
            }
        }
        Ok(())
    });

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    let (ev_sender, ev_receiver) = std::sync::mpsc::channel();

    let mut tui = Tui::new(
        terminal,
        EventHandler {
            sender: ev_sender.clone(),
            receiver: ev_receiver,
        },
    );

    tui.init()?;

    tokio::task::spawn_blocking(move || run_app(recv, tx_request, rx_response, tui).unwrap());

    // blog post does not work in separate thread
    look_for_events(100, ev_sender).await?;

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
