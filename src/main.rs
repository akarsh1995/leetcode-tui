use leetcode_tui_rs::app_ui::async_task_channel::request_channel;
use leetcode_tui_rs::app_ui::async_task_channel::ChannelRequestSender;
use leetcode_tui_rs::app_ui::tui::Tui;
use leetcode_tui_rs::config::Config;
use leetcode_tui_rs::entities::QuestionEntity;
use leetcode_tui_rs::errors::AppResult;
use leetcode_tui_rs::errors::LcAppError;
use sea_orm::Database;
use tokio::task::JoinHandle;

use leetcode_tui_rs::app_ui::app::App;
use leetcode_tui_rs::app_ui::event::{
    look_for_events, vim_ping_channel, Event, EventHandler, VimPingSender,
};

use leetcode_tui_rs::utils::{
    async_tasks_executor, do_migrations, get_config, get_reqwest_client, update_database_questions,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

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
    let client = client.clone();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    let (ev_sender, ev_receiver) = std::sync::mpsc::channel();
    let ev_sender_clone = ev_sender.clone();

    let task_receiver_from_app: JoinHandle<AppResult<()>> = tokio::spawn(async move {
        async_tasks_executor(rx_request, ev_sender_clone, &client, &database_client).await?;
        Ok(())
    });

    let tui = Tui::new(
        terminal,
        EventHandler {
            sender: ev_sender.clone(),
            receiver: ev_receiver,
        },
    );

    let vim_running = Arc::new(AtomicBool::new(false));
    let vim_running_loop_ref = vim_running.clone();
    let (vim_tx, vim_rx) = vim_ping_channel(10);
    let (should_stop_looking_for_events, should_stop_looking_events_rx) =
        tokio::sync::oneshot::channel();

    tokio::task::spawn_blocking(move || {
        run_app(
            tx_request,
            tui,
            vim_tx,
            vim_running,
            config,
            should_stop_looking_for_events,
        )
        .unwrap();
    });

    // blog post does not work in separate thread
    match look_for_events(
        5000,
        ev_sender,
        vim_running_loop_ref,
        vim_rx,
        should_stop_looking_events_rx,
    )
    .await
    {
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
    mut tui: Tui,
    vim_tx: VimPingSender,
    vim_running: Arc<AtomicBool>,
    config: Config,
    stop_events_tx: tokio::sync::oneshot::Sender<bool>,
) -> AppResult<()> {
    let config = Rc::new(config);
    tui.init()?;
    let mut app = App::new(tx_request, vim_tx, vim_running, config)?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;

        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(None)?,
            Event::Key(key_event) => app.handle_key_events(key_event)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) | Event::Redraw => tui.reinit()?,
            Event::TaskResponse(response) => app.tick(Some(*response))?,
        }
    }

    // Exit the user interface.
    tui.exit()?;
    stop_events_tx
        .send(true)
        .map_err(|e| LcAppError::StopEventsSignalSendError(e.to_string()))?;
    Ok(())
}
