use ratatui::widgets::ListState;

use crate::entities::topic_tag::Model as TopicTagModel;
use crate::{entities::question::Model as QuestionModel, errors::AppResult};
use std::collections::{HashMap, HashSet};

use super::{
    channel::{ChannelRequestSender, ChannelResponseReceiver, TaskResponse},
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

    pub questions_list: Option<HashMap<TopicTagModel, Vec<QuestionModel>>>,

    pub widget_switcher: i32,

    pub last_response: Option<TaskResponse>,

    pub show_popup: bool,

    pub task_request_sender: ChannelRequestSender,

    pub task_response_recv: ChannelResponseReceiver,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(
        wid: &'a mut Vec<Widget<'a>>,
        task_request_sender: ChannelRequestSender,
        task_response_recv: ChannelResponseReceiver,
    ) -> AppResult<Self> {
        task_request_sender.send(super::channel::TaskRequest::GetAllQuestionsMap)?;
        task_request_sender.send(super::channel::TaskRequest::GetAllTopicTags)?;

        let mut app = Self {
            running: true,
            questions_list: None,
            widgets: wid,
            widget_switcher: 0,
            task_request_sender,
            task_response_recv,
            last_response: None,
            show_popup: false,
        };
        app.update_question_list();
        Ok(app)
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
                        self.task_request_sender.send(
                            super::channel::TaskRequest::QuestionDetail { slug: slug.clone() },
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn update_question_list(&mut self) {
        let mut tt_model: Option<TopicTagModel> = None;

        match &self.widgets[self.widget_switcher as usize] {
            super::app::Widget::TopicTagList(ttl) => {
                if let Some(selected_widget) = ttl.get_selected_item() {
                    tt_model = Some(selected_widget.clone());
                }
            }
            _ => {}
        }

        for w in self.widgets.iter_mut() {
            if let Widget::QuestionList(ql) = w {
                if let Some(selected_tt_model) = &tt_model {
                    let mut items;
                    if let Some(tt_ql_map) = &mut self.questions_list {
                        if selected_tt_model.id.as_str() == "all" {
                            let set = tt_ql_map
                                .values()
                                .flat_map(|q| q.clone())
                                .collect::<HashSet<_>>();
                            items = set.into_iter().collect::<Vec<_>>();
                        } else {
                            items = tt_ql_map.get(selected_tt_model).unwrap().clone();
                        }
                        items.sort();
                        ql.items = items;
                        ql.state = ListState::default();
                    }
                }
            }
        }
    }

    // pub fn find_widget(&mut self, wid_type: Widget) -> &mut Widget {
    //     for wid in self.widgets {
    //         if wid == Widget::
    //     }

    // }

    pub fn toggle_popup(&mut self) {
        self.show_popup = !self.show_popup;
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let Ok(response) = self.task_response_recv.try_recv() {
            match response {
                TaskResponse::GetAllQuestionsMap(map) => {
                    if let Some(ql) = &mut self.questions_list {
                        ql.extend(map.into_iter())
                    } else {
                        self.questions_list = Some(map);
                    }
                    self.update_question_list()
                }
                TaskResponse::AllTopicTags(tts) => {
                    for w in self.widgets.iter_mut() {
                        match w {
                            Widget::TopicTagList(tt_list) => {
                                tt_list.items.extend(tts);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                response => self.last_response = Some(response),
            }
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
