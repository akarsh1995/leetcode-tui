use app_core::content::MainContent;
use app_core::input::Input;
use app_core::popup::{Popup, SelectPopup};

pub struct Ctx {
    pub content: MainContent,
    pub popup: Popup,
    pub select_popup: SelectPopup<String>,
    pub input: Input,
}

impl Ctx {
    pub(super) async fn new() -> Self {
        Self {
            content: MainContent::new().await,
            popup: Default::default(),
            select_popup: Default::default(),
            input: Default::default(),
        }
    }
}
