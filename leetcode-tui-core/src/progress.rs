pub struct ProgressCtx {
    pub(crate) progress: u32,
    pub(crate) total: u32,
    pub(crate) title: String,
    pub(crate) is_visible: bool,
}

impl Default for ProgressCtx {
    fn default() -> Self {
        Self {
            progress: 0,
            total: 100,
            title: "Nothing".into(),
            is_visible: false,
        }
    }
}

impl ProgressCtx {
    pub fn set_progress(&mut self, title: String, progress: u32, total: u32) {
        self.progress = progress;
        self.total = total;
        self.title = title;
        if self.progress == total || self.progress == 0 {
            self.is_visible = false;
        } else {
            self.is_visible = true;
        }
    }

    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    pub fn get_progress(&self) -> u32 {
        self.progress
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}
