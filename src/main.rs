use leetcode_tui_rs::app_ui::channel::{request_channel, response_channel};
use leetcode_tui_rs::app_ui::channel::{ChannelRequestSender, ChannelResponseReceiver};
use leetcode_tui_rs::app_ui::list::StatefulList;
use leetcode_tui_rs::app_ui::tui::Tui;
use leetcode_tui_rs::config::Config;
use leetcode_tui_rs::entities::{QuestionEntity, QuestionModel};
use leetcode_tui_rs::errors::AppResult;
use sea_orm::Database;
use tokio::task::JoinHandle;

use leetcode_tui_rs::app_ui::app::{App, Widget};
use leetcode_tui_rs::app_ui::event::{look_for_events, Event, EventHandler};
use leetcode_tui_rs::app_ui::handler::handle_key_events;
use leetcode_tui_rs::entities::topic_tag::Model as TopicTagModel;
use leetcode_tui_rs::utils::{
    do_migrations, get_config, get_reqwest_client, tasks_executor, update_database_questions,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stderr};

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

    if QuestionEntity::get_question_count(&database_client).await? == 0 {
        println!("Updating database with leetcode questions as the database is empty.");
        update_database_questions(&client, &database_client).await?;
    }

    let database_client = database_client_clone;
    let client = client_clone;

    let (tx_request, rx_request) = request_channel();
    let (tx_response, rx_response) = response_channel();
    let client = client.clone();

    let task_receiver_from_app: JoinHandle<AppResult<()>> = tokio::spawn(async move {
        tasks_executor(rx_request, tx_response, &client, &database_client).await?;
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
    tokio::task::spawn_blocking(move || run_app(tx_request, rx_response, tui).unwrap());

    // blog post does not work in separate thread
    match look_for_events(100, ev_sender).await {
        Ok(_) => Ok(()),
        Err(e) => match e {
            leetcode_tui_rs::errors::LcAppError::SyncSendError(_) => Ok(()),
            _ => Err(e),
        },
    }?;

    task_receiver_from_app.await??;

    Ok(())
}

fn run_app(
    tx_request: ChannelRequestSender,
    rx_response: ChannelResponseReceiver,
    mut tui: Tui<CrosstermBackend<Stderr>>,
) -> AppResult<()> {
    let topic_tags: Vec<TopicTagModel> = vec![TopicTagModel {
        name: Some("All".to_string()),
        id: "all".to_string(),
        slug: Some("all".to_string()),
    }];

    let questions = vec![];

    let mut qm: StatefulList<QuestionModel> = StatefulList::with_items(questions);
    let mut ttm: StatefulList<TopicTagModel> = StatefulList::with_items(topic_tags);
    let question_stateful = Widget::QuestionList(&mut qm);
    let topic_tag_stateful = Widget::TopicTagList(&mut ttm);
    let mut vw = vec![topic_tag_stateful, question_stateful];

    let mut app = App::new(&mut vw, tx_request, rx_response)?;

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
