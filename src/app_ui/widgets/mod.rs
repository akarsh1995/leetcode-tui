pub(crate) mod help_bar;
pub(crate) mod notification;
pub(crate) mod popup;
pub mod question_list;
pub mod stats;
pub mod topic_list;

use std::{collections::VecDeque, fmt::Debug, io::Stderr};

use crossterm::event::{KeyCode, KeyEvent};
use indexmap::IndexSet;
use ratatui::{prelude::Rect, prelude::*, Frame};

use crate::errors::AppResult;

use self::notification::{NotifContent, Notification, WidgetName, WidgetVariant};

use super::{
    async_task_channel::{ChannelRequestSender, TaskResponse},
    components::help_text::HelpText,
};

#[derive(Debug)]
pub struct CommonState {
    pub widget_name: WidgetName,
    active: bool,
    pub task_sender: ChannelRequestSender,
    pub is_navigable: bool,
    help_texts: IndexSet<HelpText>,
    pub notification_queue: VecDeque<Notification>,
    pub parent_can_handle_events: bool,
}

impl CommonState {
    pub(crate) fn new(
        id: WidgetName,
        task_sender: ChannelRequestSender,
        help_texts: Vec<HelpText>,
    ) -> Self {
        Self {
            widget_name: id,
            active: false,
            task_sender,
            is_navigable: true,
            help_texts: IndexSet::from_iter(help_texts),
            notification_queue: Default::default(),
            parent_can_handle_events: true,
        }
    }

    pub(crate) fn get_key_set(&self) -> IndexSet<&KeyCode> {
        self.help_texts
            .iter()
            .flat_map(|ht| ht.get_keys())
            .collect::<IndexSet<_>>()
    }
}

pub trait CommonStateManager: Debug {
    fn get_help_text_notif(&self) -> AppResult<Option<Notification>> {
        Ok(Some(Notification::HelpText(NotifContent {
            src_wid: self.get_common_state().widget_name.clone(),
            dest_wid: WidgetName::HelpLine,
            content: self.get_help_texts().clone(),
        })))
    }

    fn can_handle_key_set(&self) -> IndexSet<&KeyCode> {
        self.get_common_state().get_key_set()
    }

    fn set_active(&mut self) -> AppResult<Option<Notification>> {
        self.get_common_state_mut().active = true;
        self.get_help_text_notif()
    }

    fn is_active(&self) -> bool {
        self.get_common_state().active
    }

    fn parent_can_handle_events(&self) -> bool {
        self.get_common_state().parent_can_handle_events
    }

    fn get_help_texts(&self) -> &IndexSet<HelpText> {
        &self.get_common_state().help_texts
    }

    fn get_help_texts_mut(&mut self) -> &mut IndexSet<HelpText> {
        &mut self.get_common_state_mut().help_texts
    }

    fn is_navigable(&self) -> bool {
        self.get_common_state().is_navigable
    }

    fn set_inactive(&mut self) {
        self.get_common_state_mut().active = false;
    }

    fn get_widget_name(&self) -> WidgetName {
        self.get_common_state().widget_name.clone()
    }

    fn get_task_sender(&self) -> &ChannelRequestSender {
        &self.get_common_state().task_sender
    }

    fn show_spinner(&mut self) -> AppResult<Notification> {
        self.spinner_notif(true)
    }

    fn hide_spinner(&mut self) -> AppResult<Notification> {
        self.spinner_notif(false)
    }

    fn spinner_notif(&mut self, show: bool) -> AppResult<Notification> {
        let src_wid = self.get_widget_name();
        Ok(Notification::Loading(NotifContent {
            src_wid,
            dest_wid: WidgetName::HelpLine,
            content: show,
        }))
    }

    // fn get_notification_queue(&mut self) -> &mut VecDeque<Notification>;

    fn get_common_state_mut(&mut self) -> &mut CommonState;

    fn get_common_state(&self) -> &CommonState;
}

macro_rules! state_manager_common_methods {
    () => {
        fn get_common_state(&self) -> &CommonState {
            &self.common_state
        }

        fn get_common_state_mut(&mut self) -> &mut CommonState {
            &mut self.common_state
        }

        // fn get_notification_queue(&mut self) -> &mut std::collections::VecDeque<Notification> {
        //     &mut self.common_state.notification_queue
        // }
    };
}

use state_manager_common_methods;

macro_rules! impl_common_state {
    ($t:ty) => {
        impl CommonStateManager for $t {
            crate::app_ui::widgets::state_manager_common_methods!();
        }
    };

    ($t:ty, $e:item) => {
        impl CommonStateManager for $t {
            crate::app_ui::widgets::state_manager_common_methods!();
            $e
        }

    }
}

pub(crate) use impl_common_state;

pub trait Widget: Debug {
    fn render(&mut self, rect: Rect, frame: &mut Frame<CrosstermBackend<Stderr>>);

    fn handler(&mut self, _event: KeyEvent) -> AppResult<Option<Notification>> {
        Ok(None)
    }

    fn process_task_response(
        &mut self,
        _response: TaskResponse,
    ) -> AppResult<Option<Notification>> {
        Ok(None)
    }

    fn setup(&mut self) -> AppResult<()> {
        Ok(())
    }

    fn process_notification(
        &mut self,
        _notification: Notification,
    ) -> AppResult<Option<Notification>> {
        Ok(None)
    }
}

macro_rules! widget_variant_arms {
    ($fn_name: ident, $i:ident, ($($arg: ident),*)) => {
        match $i {
            WidgetVariant::QuestionList(v) => v.$fn_name($($arg),*),
            WidgetVariant::TopicList(v) => v.$fn_name($($arg),*),
            WidgetVariant::Stats(v) => v.$fn_name($($arg),*),
            WidgetVariant::HelpLine(v) => v.$fn_name($($arg),*),
        }
    };
}

macro_rules! gen_methods {
(
    $(
        ($fn_name:ident,  ($(($arg:ident, $par_type:ty)),*), $res:ty)
    ),*
) => {
        $(
            pub fn $fn_name(&mut self, $($arg: $par_type),*) -> $res {
                widget_variant_arms!($fn_name, self, ($($arg),*))
            }
        )*
    };

(
    $(
        ($fn_name:ident, $_:ident, ($(($arg:ident, $par_type:ty)),*), $res:ty)
    ),*
) => {
        $(
            pub fn $fn_name(&self, $($arg: $par_type),*) -> $res {
                widget_variant_arms!($fn_name, self, ($($arg),*))
            }
        )*
    };
}

impl WidgetVariant {
    gen_methods!(
        (parent_can_handle_events, nm, (), bool),
        (is_navigable, nm, (), bool)
    );
    gen_methods!(
        // (get_notification_queue, (), &mut VecDeque<Notification>),
        (set_active, (), AppResult<Option<Notification>>),
        (set_inactive, (), ()),
        (setup, (), AppResult<()>),
        (
            process_task_response,
            ((response, TaskResponse)),
            AppResult<Option<Notification>>
        ),
        (
            handler,
            ((event, KeyEvent)),
            AppResult<Option<Notification>>
        ),
        (
            process_notification,
            ((notification, Notification)),
            AppResult<Option<Notification>>
        ),
        (
            render,
            ((rect, Rect), (frame, &mut Frame<CrosstermBackend<Stderr>>)),
            ()
        )
    );
}

pub type CrosstermStderr<'a> = Frame<'a, CrosstermBackend<Stderr>>;
