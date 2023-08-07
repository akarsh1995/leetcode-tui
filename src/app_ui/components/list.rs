use ratatui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct StatefulList<T, F: Fn(&T) -> &str> {
    pub state: ListState,
    pub items: Vec<T>,
    pub str_fn: Option<F>,
}

impl<T, F> Default for StatefulList<T, F> {
    fn default() -> Self {
        Self {
            state: ListState::default(),
            items: vec![],
            str_fn: None,
        }
    }
}

impl<T, F> StatefulList<T, F> {
    pub fn add_item(&mut self, item: T) {
        if self.items.is_empty() {
            self.state.select(Some(0))
        }
        self.items.push(item)
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        match self.state.selected() {
            Some(i) => Some(&self.items[i]),
            None => None,
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T, F> {
        let mut list_state = ListState::default();
        if !items.is_empty() {
            list_state.select(Some(0))
        }
        StatefulList {
            state: list_state,
            items,
            str_fn: None,
        }
    }

    pub fn next(&mut self) {
        let mut b = self.items.len() as i32;
        if b == 0 {
            b = 1;
        }
        let i = match self.state.selected() {
            Some(i) => {
                let a = i as i32 + 1;
                ((a % b) + b) % b
            }
            None => 0,
        };
        self.state.select(Some(i as usize));
    }

    pub fn previous(&mut self) {
        let mut b = self.items.len() as i32;
        if b == 0 {
            b = 1;
        }
        let i = match self.state.selected() {
            Some(i) => {
                let a = i as i32 - 1;
                ((a % b) + b) % b
            }
            None => 0,
        };
        self.state.select(Some(i as usize));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
