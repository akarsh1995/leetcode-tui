use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use leetcode_tui_rs::app_ui::channel::{self, request_channel, response_channel};
use leetcode_tui_rs::app_ui::channel::{
    ChannelRequestSender, ChannelResponseReceiver, Request, Response,
};
use leetcode_tui_rs::app_ui::list::StatefulList;
use leetcode_tui_rs::app_ui::tui::Tui;
use leetcode_tui_rs::app_ui::ui::render;
use leetcode_tui_rs::config::{self, Config};
use leetcode_tui_rs::db_ops::ModelUtils;
use leetcode_tui_rs::deserializers;
use leetcode_tui_rs::deserializers::question::{ProblemSetQuestionListQuery, Question};
use leetcode_tui_rs::deserializers::question_content::{QueryQuestionContent, QuestionContent};
use leetcode_tui_rs::entities::QuestionModel;
use leetcode_tui_rs::graphql::problemset_question_list::Query;
use leetcode_tui_rs::graphql::{question_content, GQLLeetcodeQuery};
use reqwest::header::{HeaderMap, HeaderValue};
use sea_orm::Database;
use tracing;
use tracing_subscriber;

use leetcode_tui_rs::app_ui::app::{App, AppResult, TTReciever, Widget};
use leetcode_tui_rs::app_ui::event::{look_for_events, Event, EventHandler};
use leetcode_tui_rs::app_ui::handler::handle_key_events;
// use leetcode_tui_rs::app_ui::tui::Tui;
use leetcode_tui_rs::entities::topic_tag::Model as TopicTagModel;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::collections::HashMap;
use std::io::{self, Stderr, Stdout};

use once_cell::sync::Lazy;

static CONFIG: Lazy<config::Config> = Lazy::new(|| Config::from_file("./leetcode.config"));

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let csrf = CONFIG.leetcode.csrftoken.as_str();
    let sess = CONFIG.leetcode.leetcode_session.as_str();
    let mut headers = HeaderMap::new();
    headers.append(
        "Cookie",
        HeaderValue::from_str(&format!("LEETCODE_SESSION={sess}; csrftoken={csrf}")).unwrap(),
    );
    reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
});

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let database_client = Database::connect(CONFIG.db.url.as_str()).await.unwrap();

    // let query = Query::default();
    // let query_response: ProblemSetQuestionListQuery = query.post(&CLIENT).await;
    // Question::multi_insert(&database_client, query_response.get_questions()).await;

    // Create an application.
    use crossbeam;

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
    let client = CLIENT.clone();

    tokio::spawn(async move {
        let mut rx_request = rx_request;
        let tx_response = tx_response;
        while let Some(task) = rx_request.recv().await {
            match task {
                Request::QuestionDetail { slug } => {
                    let query: deserializers::question_content::Data =
                        question_content::Query::new(slug).post(&client).await;
                    tx_response
                        .send(channel::Response::QuestionDetail(query.data.question))
                        .unwrap();
                }
            }
        }
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

    look_for_events(100, ev_sender).await;

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
