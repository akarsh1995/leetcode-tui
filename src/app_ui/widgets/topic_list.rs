use crate::{
    app_ui::{
        async_task_channel::{
            ChannelRequestSender, Request as TaskRequestFormat, Response, TaskRequest, TaskResponse,
        },
        components::{color::TokyoNightColors, help_text::CommonHelpText, list::StatefulList},
        widgets::question_list::custom_lists::NEETCODE_75,
    },
    entities::TopicTagModel,
    errors::AppResult,
};

use crossterm::event::KeyEvent;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use super::{
    notification::{
        NotifContent, Notification,
        WidgetName::{self, QuestionList},
    },
    CommonState, CommonStateManager, CrosstermStderr, Widget,
};
use crate::app_ui::components::color::Callout;

#[derive(Debug)]
pub struct TopicTagListWidget {
    common_state: CommonState,
    pub topics: StatefulList<TopicTagModel>,
}

impl TopicTagListWidget {
    pub fn new(id: WidgetName, task_sender: ChannelRequestSender) -> Self {
        Self {
            common_state: CommonState::new(
                id,
                task_sender,
                vec![
                    CommonHelpText::ScrollUp.into(),
                    CommonHelpText::ScrollDown.into(),
                    CommonHelpText::SwitchPane.into(),
                ],
            ),
            topics: Default::default(),
        }
    }
}

impl TopicTagListWidget {
    fn get_item(ttm: &TopicTagModel) -> ListItem {
        ListItem::new(Text::styled(ttm.name.clone(), Style::default()))
    }

    fn update_questions(&mut self) -> AppResult<Option<Notification>> {
        if let Some(topic_tag) = self.topics.get_selected_item() {
            let questions = vec![topic_tag.clone()];
            let notif = Notification::Questions(NotifContent::new(
                WidgetName::TopicList,
                QuestionList,
                questions,
            ));
            return Ok(Some(notif));
        }
        Ok(None)
    }
}

super::impl_common_state!(
    TopicTagListWidget,
    fn set_active(&mut self) -> AppResult<Option<Notification>> {
        self.common_state.active = true;
        Ok(Some(Notification::HelpText(NotifContent::new(
            WidgetName::TopicList,
            WidgetName::HelpLine,
            self.get_help_texts().clone(),
        ))))
    }
);

impl Widget for TopicTagListWidget {
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let lines = self
            .topics
            .items
            .iter()
            .map(Self::get_item)
            .collect::<Vec<_>>();

        let mut border_style = Style::default();

        if self.is_active() {
            border_style = border_style.fg(TokyoNightColors::Pink.into());
        }

        let hstyle: Style = Callout::Info.into();
        let items = List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Topics")
                    .border_style(border_style),
            )
            .highlight_style(
                hstyle
                    .add_modifier(Modifier::BOLD)
                    .fg(TokyoNightColors::Pink.into())
                    .bg(TokyoNightColors::Selection.into()),
            );
        frame.render_stateful_widget(items, rect, &mut self.topics.state);
    }

    fn handler(&mut self, event: KeyEvent) -> AppResult<Option<Notification>> {
        match event.code {
            crossterm::event::KeyCode::Up => {
                self.topics.previous();
                return self.update_questions();
            }
            crossterm::event::KeyCode::Down => {
                self.topics.next();
                return self.update_questions();
            }
            _ => {}
        };
        Ok(None)
    }

    fn process_task_response(&mut self, response: TaskResponse) -> AppResult<Option<Notification>> {
        if let TaskResponse::AllTopicTags(Response { content, .. }) = response {
            self.topics.add_item(TopicTagModel {
                name: "All".to_owned(),
                id: "all".to_owned(),
                slug: "all".to_owned(),
            });
            self.topics.add_item(NEETCODE_75.get_topic_tag());
            for tt in content {
                self.topics.add_item(tt)
            }
        }
        self.update_questions()?;
        Ok(None)
    }

    fn setup(&mut self) -> AppResult<()> {
        self.get_task_sender()
            .send(TaskRequest::GetAllTopicTags(TaskRequestFormat {
                widget_name: self.get_widget_name(),
                request_id: "".to_string(),
                content: (),
            }))
            .map_err(Box::new)?;
        Ok(())
    }
}
