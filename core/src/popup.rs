use ratatui::widgets::ScrollbarState;

#[derive(Default)]
pub struct Popup {
    pub visible: bool,
    lines: Vec<String>,
    pub v_scroll_state: ScrollbarState,
    pub v_scroll: u16,
}

impl Popup {
    pub fn new(lines: Vec<String>) -> Self {
        let mut p = Popup {
            lines,
            ..Default::default()
        };
        p.v_scroll_state = p.v_scroll_state.content_length(p.lines.len() as u16);
        p
    }

    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        true
    }

    pub fn get_text(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn set_lines(&mut self, lines: Vec<String>) {
        let mut p = Self::new(lines);
        p.visible = self.visible;
        *self = p;
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn scroll_down(&mut self) -> bool {
        if self.v_scroll == self.lines.len().saturating_sub(1) as u16 {
            return false;
        }
        self.v_scroll = self.v_scroll.saturating_add(1);
        self.v_scroll_state = self.v_scroll_state.position(self.v_scroll);
        true
    }

    pub fn scroll_up(&mut self) -> bool {
        if self.v_scroll == 0 {
            return false;
        }
        self.v_scroll = self.v_scroll.saturating_sub(1);
        self.v_scroll_state = self.v_scroll_state.position(self.v_scroll);
        true
    }
}
