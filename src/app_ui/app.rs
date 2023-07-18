use ratatui::widgets::ListState;

use crate::entities::topic_tag::Model as TopicTagModel;
use crate::{entities::question::Model as QuestionModel, errors::AppResult};
use std::collections::{HashMap, HashSet};

use super::{
    channel::{ChannelRequestSender, ChannelResponseReceiver, Response},
    list::StatefulList,
};

/// Application result type.

pub type SS = (TopicTagModel, Vec<QuestionModel>);

pub type TTReciever = crossbeam::channel::Receiver<SS>;
pub type TTSender = crossbeam::channel::Sender<SS>;

#[derive(Debug)]
pub enum Widget<'a> {
    QuestionList(&'a mut StatefulList<QuestionModel>),
    TopicTagList(&'a mut StatefulList<TopicTagModel>),
}

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,

    pub widgets: &'a mut Vec<Widget<'a>>,

    pub questions_list: &'a HashMap<String, Vec<QuestionModel>>,

    pub widget_switcher: i32,

    pub last_response: Option<Response>,

    pub show_popup: bool,

    pub task_request_sender: ChannelRequestSender,

    pub task_response_recv: ChannelResponseReceiver,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(
        wid: &'a mut Vec<Widget<'a>>,
        questions_list: &'a HashMap<String, Vec<QuestionModel>>,
        task_request_sender: ChannelRequestSender,
        task_response_recv: ChannelResponseReceiver,
    ) -> Self {
        let mut app = Self {
            running: true,
            questions_list,
            widgets: wid,
            widget_switcher: 0,
            task_request_sender,
            task_response_recv,
            last_response: None,
            show_popup: false,
        };
        app.update_question_list();
        app
    }

    pub fn next_widget(&mut self) {
        let a = self.widget_switcher + 1;
        let b = self.widgets.len() as i32;
        self.widget_switcher = ((a % b) + b) % b;
    }

    pub fn prev_widget(&mut self) {
        let a = self.widget_switcher - 1;
        let b = self.widgets.len() as i32;
        self.widget_switcher = ((a % b) + b) % b;
    }

    pub fn get_current_widget(&self) -> &Widget {
        &self.widgets[self.widget_switcher as usize]
    }

    pub fn update_question_in_popup(&self) -> AppResult<()> {
        if self.show_popup {
            if let Widget::QuestionList(s) = self.get_current_widget() {
                if let Some(selected_item) = s.get_selected_item() {
                    if let Some(slug) = &selected_item.title_slug {
                        self.task_request_sender
                            .send(super::channel::Request::QuestionDetail { slug: slug.clone() })?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn update_question_list(&mut self) {
        let mut name: Option<String> = None;

        match &self.widgets[self.widget_switcher as usize] {
            super::app::Widget::TopicTagList(ttl) => {
                if let Some(selected_widget) = ttl.get_selected_item() {
                    if let Some(n) = &selected_widget.name {
                        name = Some(n.clone());
                    }
                }
            }
            _ => {}
        }

        for w in self.widgets.iter_mut() {
            if let Widget::QuestionList(ql) = w {
                if let Some(name) = &name {
                    let mut items;
                    if name.as_str() == "All" {
                        let set = self
                            .questions_list
                            .values()
                            .flat_map(|q| q.clone())
                            .collect::<HashSet<_>>();
                        items = set.into_iter().map(|c| c.clone()).collect::<Vec<_>>();
                    } else {
                        items = self.questions_list.get(name).unwrap().clone();
                    }
                    items.sort();
                    ql.items = items;
                    ql.state = ListState::default();
                }
            }
        }
    }

    pub fn toggle_popup(&mut self) {
        self.show_popup = !self.show_popup;
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let Ok(response) = self.task_response_recv.try_recv() {
            self.last_response = Some(response);
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
