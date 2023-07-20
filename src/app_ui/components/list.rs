use ratatui::widgets::ListState;
use std::rc::Rc;

#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<Rc<T>>,
}

impl<T> StatefulList<T> {
    pub fn add_item(&mut self, item: T) {
        if self.items.len() == 0 {
            self.state.select(Some(0))
        }
        self.items.push(Rc::new(item))
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        match self.state.selected() {
            Some(i) => Some(&self.items[i]),
            None => None,
        }
    }

    pub fn with_items(items: Vec<Rc<T>>) -> StatefulList<T> {
        let mut list_state = ListState::default();
        if !items.is_empty() {
            list_state.select(Some(0))
        }
        StatefulList {
            state: list_state,
            items,
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
