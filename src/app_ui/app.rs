use std::collections::{HashMap, VecDeque};

use crate::errors::AppResult;

use super::channel::{ChannelRequestSender, ChannelResponseReceiver};
// use super::widgets::footer::Footer;
use super::widgets::notification::{Notification, WidgetName, WidgetVariant};
use super::widgets::question_list::QuestionListWidget;
// use super::widgets::stats::Stats;
use super::widgets::topic_list::TopicTagListWidget;
use super::widgets::WidgetList;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    widget_map: HashMap<WidgetName, WidgetVariant>,

    selected_wid_idx: i32,

    widgets: Vec<WidgetName>,

    pub popups: WidgetList,

    pub task_request_sender: ChannelRequestSender,

    pub task_response_recv: ChannelResponseReceiver,

    pub pending_notifications: VecDeque<Option<Notification>>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(
        task_request_sender: ChannelRequestSender,
        task_response_recv: ChannelResponseReceiver,
    ) -> AppResult<Self> {
        let w0 = WidgetVariant::TopicList(TopicTagListWidget::new(0, task_request_sender.clone()));
        let w1 =
            WidgetVariant::QuestionList(QuestionListWidget::new(1, task_request_sender.clone()));

        let order = [(WidgetName::TopicList, w0), (WidgetName::QuestionList, w1)];

        let widget_order = order.iter().map(|w| w.0.clone()).collect::<Vec<_>>();

        let mut app = Self {
            running: true,
            widget_map: HashMap::from(order),
            selected_wid_idx: 0,
            task_request_sender,
            task_response_recv,
            popups: vec![],
            widgets: widget_order,
            pending_notifications: vec![].into(),
        };
        app.setup()?;
        Ok(app)
    }

    pub fn widgets(&self) -> &Vec<WidgetName> {
        &self.widgets
    }

    pub fn has_popups(&self) -> bool {
        !self.popups.is_empty()
    }

    pub fn navigate(&mut self, val: i32) {
        if self.has_popups() {
            return;
        }
        self.get_current_widget_mut().set_inactive();
        let a = self.selected_wid_idx + val;
        let b = self.widgets.len() as i32;
        self.selected_wid_idx = ((a % b) + b) % b;
        self.get_current_widget_mut().set_active();
        if !self.get_current_widget().is_navigable() {
            self.navigate(val)
        }
    }

    pub fn get_widget(&mut self, v: &WidgetName) -> &mut WidgetVariant {
        self.widget_map.get_mut(v).unwrap()
    }

    pub fn next_widget(&mut self) {
        self.navigate(1);
    }

    pub fn prev_widget(&mut self) {
        self.navigate(-1);
    }

    pub fn get_current_widget(&self) -> &WidgetVariant {
        &self
            .widget_map
            .get(&self.widgets[self.selected_wid_idx as usize])
            .unwrap()
    }

    pub fn get_current_widget_mut(&mut self) -> &mut WidgetVariant {
        self.widget_map
            .get_mut(&self.widgets[self.selected_wid_idx as usize])
            .unwrap()
    }

    pub fn setup(&mut self) -> AppResult<()> {
        self.get_current_widget_mut().set_active();
        let mut v = vec![];
        for wid in self.widgets().clone() {
            let k = self.widget_map.get_mut(&wid).unwrap().setup()?;
            v.push(k);
        }
        self.pending_notifications.extend(v);
        Ok(())
    }

    pub fn get_new_id(&self) -> i32 {
        (self.widgets.len() + self.popups.len()) as i32
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) -> AppResult<()> {
        // pop the popup out of popup stack
        if let Some(popup) = self.popups.last() {
            if !popup.is_active() {
                self.popups.pop();
            }
        }
        self.check_for_task()?;
        self.process_pending_notification()?;
        Ok(())
    }

    fn check_for_task(&mut self) -> AppResult<()> {
        if let Ok(task_result) = self.task_response_recv.try_recv() {
            let wid_name = self.widgets[task_result.get_sender_id() as usize].clone();
            self.pending_notifications.push_back(
                self.widget_map
                    .get_mut(&wid_name)
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
                let widget_var = self.widget_map.get_mut(wid_name).unwrap();
                let more_notif = widget_var.process_notification(&notif)?;
                self.pending_notifications.push_back(more_notif);
            }
        }
        Ok(())
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
