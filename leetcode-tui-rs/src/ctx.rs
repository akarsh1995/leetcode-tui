use leetcode_tui_core::content::MainContent;
use leetcode_tui_core::help::Help;
use leetcode_tui_core::input::Input;
use leetcode_tui_core::popup::{Popup, SelectPopup};

pub struct Ctx {
    pub content: MainContent,
    pub popup: Popup,
    pub select_popup: SelectPopup<String>,
    pub input: Input,
    pub help: Help,
}

impl Ctx {
    pub(super) async fn new() -> Self {
        Self {
            content: MainContent::new().await,
            popup: Default::default(),
            select_popup: Default::default(),
            input: Default::default(),
            help: Default::default(),
        }
    }
}
