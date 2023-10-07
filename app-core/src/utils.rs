use shared::tui::Term;
use std::ops::Range;

use crate::emit;
const HELP_MARGIN: u16 = 1;
const TOP_MARGIN: u16 = 1;

pub struct Paginate<T> {
    list: Vec<T>,
    nth_window: usize,
    cursor_range: Range<usize>,
    cursor: usize,
    hovered: Option<T>,
}

impl<T> Paginate<T>
where
    T: Clone,
{
    pub fn new(list: Vec<T>) -> Self {
        let hovered = list.first().cloned();
        Self {
            list,
            nth_window: Default::default(),
            cursor_range: Default::default(),
            cursor: Default::default(),
            hovered,
        }
    }

    pub fn update_list(&mut self, list: Vec<T>) {
        *self = Self::new(list)
    }
}

impl<T> Paginate<T>
where
    T: Clone,
{
    pub fn next_elem(&mut self) -> bool {
        if self.list.is_empty() {
            emit!(Popup(vec!["List is empty".into()]));
            return true;
        }
        self.set_cursor_range();
        let old_cursor = self.cursor;
        let old_window = self.nth_window;

        if self.cursor >= self.cursor_range.end {
            self.nth_window = self
                .nth_window
                .saturating_add(1)
                .min(self.list.windows(self.cursor_upper_bound()).count() - 1);
        } else {
            self.cursor = (self.cursor + 1).min(self.cursor_upper_bound() - 1);
        }
        self.hovered = self.window().get(self.cursor).cloned();
        self.cursor != old_cursor || self.nth_window != old_window
    }

    pub fn prev_elem(&mut self) -> bool {
        if self.list.is_empty() {
            emit!(Popup(vec!["List is empty".into()]));
            return true;
        }
        self.set_cursor_range();
        let old_cursor = self.cursor;
        let old_window = self.nth_window;

        if self.cursor < self.cursor_range.start {
            self.nth_window = self.nth_window.saturating_sub(1);
        } else {
            self.cursor = self.cursor.saturating_sub(1);
        }

        self.hovered = self.window().get(self.cursor).cloned();
        self.cursor != old_cursor || self.nth_window != old_window
    }

    fn set_cursor_range(&mut self) {
        let b;
        if self.cursor_upper_bound() < self.term_height() {
            b = self.cursor_upper_bound()
        } else {
            b = self.cursor_upper_bound().saturating_sub(3)
        }
        if self.nth_window == 0 {
            //first window
            self.cursor_range = 0..b;
        } else if self.nth_window == (self.list.windows(self.cursor_upper_bound()).count() - 1) {
            // last_window
            self.cursor_range = 3..self.cursor_upper_bound();
        } else {
            self.cursor_range = 3..self.cursor_upper_bound() - 3;
        }
    }

    fn cursor_upper_bound(&self) -> usize {
        self.term_height().min(self.list.len())
    }

    fn term_height(&self) -> usize {
        (Term::size().rows - HELP_MARGIN - TOP_MARGIN) as usize
    }

    pub fn window(&self) -> &[T] {
        self.list
            .windows(self.cursor_upper_bound())
            .nth(self.nth_window)
            .unwrap()
    }
    pub fn hovered(&self) -> Option<&T> {
        self.hovered.as_ref()
    }
}
