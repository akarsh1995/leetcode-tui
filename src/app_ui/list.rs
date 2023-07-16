use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn add_item(&mut self, item: T) {
        self.items.push(item)
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        match self.state.selected() {
            Some(i) => Some(&self.items[i]),
            None => None,
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                let a = i as i32 + 1;
                let b = self.items.len() as i32;
                ((a % b) + b) % b
            }
            None => 0,
        };
        self.state.select(Some(i as usize));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                let a = i as i32 - 1;
                let b = self.items.len() as i32;
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
