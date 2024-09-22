use ratatui::widgets::*;

pub struct Help {
    state: TableState,
    items: Vec<Vec<&'static str>>,
    visible: bool,
}

impl Default for Help {
    fn default() -> Self {
        let mut help = Self {
            state: TableState::default(),
            items: vec![
                vec!["t", "Move to Next Topic"],
                vec!["T", "Move to Previous Topic"],
                vec!["Ctrl+s", "Show/Hide topic stats"],
                vec!["j/Down", "Move to Next Question"],
                vec!["k/Up", "Move to Previous Question"],
                vec!["r", "Move to Random Question"],
                vec!["Enter", "Read Question/Selection"],
                vec!["e", "Open Editor"],
                vec!["R", "Run Solution"],
                vec!["s", "Submit Solution"],
                vec!["/", "Search"],
                vec!["c", "Open config file"],
            ],
            visible: Default::default(),
        };
        if !help.items.is_empty() {
            help.state.select(Some(0));
        }
        help
    }
}

impl Help {
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    pub fn get_mut_state(&mut self) -> &mut TableState {
        &mut self.state
    }
}

impl Help {
    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        true
    }

    pub fn get_items(&self) -> &Vec<Vec<&'static str>> {
        &self.items
    }

    pub fn next(&mut self) -> bool {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        true
    }

    pub fn previous(&mut self) -> bool {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        true
    }

    pub fn get_headers() {}
}
