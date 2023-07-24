use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use super::channel::{ChannelRequestSender, ChannelResponseReceiver};
use super::event::VimPingSender;
use super::widgets::footer::Footer;
use super::widgets::notification::{Notification, WidgetName, WidgetVariant};
use super::widgets::popup::Popup;
use super::widgets::question_list::QuestionListWidget;
use super::widgets::stats::Stats;
use super::widgets::topic_list::TopicTagListWidget;
use super::widgets::Widget;
use crate::errors::AppResult;
use indexmap::IndexMap;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub widget_map: indexmap::IndexMap<WidgetName, WidgetVariant>,

    selected_wid_idx: i32,

    pub task_request_sender: ChannelRequestSender,

    pub task_response_recv: ChannelResponseReceiver,

    pub pending_notifications: VecDeque<Option<Notification>>,

    pub popup_stack: Vec<Popup>,

    pub vim_tx: VimPingSender,

    pub vim_running: Arc<AtomicBool>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(
        task_request_sender: ChannelRequestSender,
        task_response_recv: ChannelResponseReceiver,
        vim_tx: VimPingSender,
        vim_running: Arc<AtomicBool>,
    ) -> AppResult<Self> {
        let w0 = WidgetVariant::TopicList(TopicTagListWidget::new(
            WidgetName::TopicList,
            task_request_sender.clone(),
        ));
        let w1 = WidgetVariant::QuestionList(QuestionListWidget::new(
            WidgetName::QuestionList,
            task_request_sender.clone(),
            vim_tx.clone(),
            vim_running.clone(),
        ));

        let w2 = WidgetVariant::Stats(Stats::new(WidgetName::Stats, task_request_sender.clone()));

        let w3 = WidgetVariant::HelpLine(Footer::new(
            WidgetName::HelpLine,
            task_request_sender.clone(),
        ));

        let order = [
            (WidgetName::TopicList, w0),
            (WidgetName::QuestionList, w1),
            (WidgetName::Stats, w2),
            (WidgetName::HelpLine, w3),
        ];

        let mut app = Self {
            running: true,
            widget_map: IndexMap::from(order),
            selected_wid_idx: 0,
            task_request_sender,
            task_response_recv,
            pending_notifications: vec![].into(),
            popup_stack: vec![],
            vim_running,
            vim_tx,
        };
        app.setup()?;
        Ok(app)
    }

    pub fn total_widgets_count(&self) -> usize {
        self.widget_map.len()
    }

    pub fn navigate(&mut self, val: i32) -> AppResult<Option<Notification>> {
        if self.get_current_popup().is_some() {
            return Ok(None);
        }
        self.get_current_widget_mut().set_inactive();
        let a = self.selected_wid_idx + val;
        let b = self.total_widgets_count() as i32;
        self.selected_wid_idx = ((a % b) + b) % b;
        let maybe_notif = self.get_current_widget_mut().set_active()?;
        self.push_notif(maybe_notif);
        if !self.get_current_widget().is_navigable() {
            self.navigate(val)?;
        }
        Ok(None)
    }

    pub fn get_widget(&mut self, v: &WidgetName) -> &mut WidgetVariant {
        self.widget_map.get_mut(v).unwrap()
    }

    pub fn next_widget(&mut self) -> AppResult<Option<Notification>> {
        self.navigate(1)
    }

    pub fn prev_widget(&mut self) -> AppResult<Option<Notification>> {
        self.navigate(-1)
    }

    pub fn get_current_widget(&self) -> &WidgetVariant {
        let (_, v) = self
            .widget_map
            .get_index(self.selected_wid_idx as usize)
            .unwrap();
        v
    }

    pub fn get_current_widget_mut(&mut self) -> &mut WidgetVariant {
        let (_, v) = self
            .widget_map
            .get_index_mut(self.selected_wid_idx as usize)
            .unwrap();
        v
    }

    pub fn get_current_popup(&self) -> Option<&Popup> {
        self.popup_stack.last()
    }

    pub fn get_current_popup_mut(&mut self) -> Option<&mut Popup> {
        self.popup_stack.last_mut()
    }

    pub fn setup(&mut self) -> AppResult<()> {
        let maybe_notif = self.get_current_widget_mut().set_active()?;
        self.push_notif(maybe_notif);
        let mut v = vec![];
        for (_, widget) in self.widget_map.iter_mut() {
            v.push(widget.setup()?);
        }
        self.pending_notifications.extend(v);
        Ok(())
    }

    pub fn push_notif(&mut self, value: Option<Notification>) {
        self.pending_notifications.push_back(value)
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) -> AppResult<()> {
        if let Some(popup) = self.get_current_popup_mut() {
            if !popup.is_active() {
                self.popup_stack.pop();
                let maybe_notif = self.get_current_widget_mut().set_active()?;
                self.push_notif(maybe_notif);
            };
        }
        self.check_for_task()?;
        self.process_pending_notification()?;
        Ok(())
    }

    fn check_for_task(&mut self) -> AppResult<()> {
        if let Ok(task_result) = self.task_response_recv.try_recv() {
            self.pending_notifications.push_back(
                self.widget_map
                    .get_mut(&task_result.get_widget_name())
                    .unwrap()
                    .process_task_response(task_result)?,
            );
        }
        Ok(())
    }

    pub fn process_pending_notification(&mut self) -> AppResult<()> {
        while let Some(elem) = self.pending_notifications.pop_front() {
            if let Some(notif) = elem {
                let wid_name = notif.get_wid_name();
                if let WidgetName::Popup = wid_name {
                    let mut popup_instance =
                        Popup::new(wid_name.clone(), self.task_request_sender.clone());
                    self.push_notif(popup_instance.set_active()?);
                    let maybe_notif = popup_instance.process_notification(&notif)?;
                    self.pending_notifications.push_back(maybe_notif);
                    self.popup_stack.push(popup_instance);
                } else {
                    let widget_var = self.widget_map.get_mut(wid_name).unwrap();
                    let more_notif = widget_var.process_notification(&notif)?;
                    self.pending_notifications.push_back(more_notif);
                }
            }
        }
        Ok(())
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
