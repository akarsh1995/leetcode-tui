use crate::errors::AppResult;

use super::channel::{ChannelRequestSender, ChannelResponseReceiver};
use super::widgets::notification::{
    Notification, NotificationRequestReceiver, NotificationRequestSender,
};
use super::widgets::question_list::QuestionListWidget;
use super::widgets::stats::Stats;
use super::widgets::topic_list::TopicTagListWidget;
use super::widgets::{self, Widget, WidgetList};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    widgets: WidgetList,

    selected_wid_idx: i32,

    pub popups: WidgetList,

    pub task_request_sender: ChannelRequestSender,

    pub task_response_recv: ChannelResponseReceiver,

    pub notification_receiver: NotificationRequestReceiver,

    notification_sender: NotificationRequestSender,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(
        task_request_sender: ChannelRequestSender,
        task_response_recv: ChannelResponseReceiver,
    ) -> AppResult<Self> {
        let (tx, rx) = super::widgets::notification::notification_channel();

        let mut app = Self {
            running: true,
            widgets: vec![
                Box::new(TopicTagListWidget::new(
                    0,
                    task_request_sender.clone(),
                    tx.clone(),
                )),
                Box::new(QuestionListWidget::new(
                    1,
                    task_request_sender.clone(),
                    tx.clone(),
                )),
                Box::new(Stats::new(2, task_request_sender.clone(), tx.clone())),
            ],
            notification_receiver: rx,
            selected_wid_idx: 0,
            task_request_sender,
            task_response_recv,
            popups: vec![],
            notification_sender: tx.clone(),
        };
        app.setup()?;
        Ok(app)
    }

    pub fn widgets(&mut self) -> &mut WidgetList {
        &mut self.widgets
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

    pub fn next_widget(&mut self) {
        self.navigate(1);
    }

    pub fn prev_widget(&mut self) {
        self.navigate(-1);
    }

    pub fn get_current_widget(&self) -> &dyn Widget {
        &*self.widgets[self.selected_wid_idx as usize]
    }

    pub fn get_current_widget_mut(&mut self) -> &mut dyn Widget {
        &mut *self.widgets[self.selected_wid_idx as usize]
    }

    pub fn setup(&mut self) -> AppResult<()> {
        self.get_current_widget_mut().set_active();
        for wid in self.widgets() {
            wid.setup()?;
        }
        Ok(())
    }

    // pub fn update_question_in_popup(&self) -> AppResult<()> {
    //     if self.show_popup {
    //         if let Widget::QuestionList(wid) = self.get_current_widget() {
    //             if let Some(selected_item) = wid.questions.get_selected_item() {
    //                 if let Some(slug) = &selected_item.title_slug {
    //                     self.task_request_sender.send(
    //                         super::channel::TaskRequest::QuestionDetail { slug: slug.clone() },
    //                     )?;
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }
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
        self.check_for_notification()?;
        self.check_for_task()?;
        Ok(())
    }

    fn check_for_task(&mut self) -> AppResult<()> {
        if let Ok(task_result) = self.task_response_recv.try_recv() {
            self.widgets[task_result.get_sender_id() as usize].process_task_response(task_result)?
        }
        Ok(())
    }

    fn check_for_notification(&mut self) -> AppResult<()> {
        if let Ok(notification) = &self.notification_receiver.try_recv() {
            match notification {
                Notification::Popup(_) => {
                    let mut popup = widgets::popup::Popup::new(
                        self.get_new_id(),
                        self.task_request_sender.clone(),
                        self.notification_sender.clone(),
                    );
                    popup.process_notification(notification)?;
                    self.popups.push(Box::new(popup));
                }
                n => {
                    for wid in self.widgets() {
                        wid.process_notification(n)?;
                    }
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
