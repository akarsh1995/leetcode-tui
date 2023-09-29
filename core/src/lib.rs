pub mod event;
pub mod step;
pub mod topic;
pub use event::Event;

pub mod question {}

pub mod utils {
    use shared::tui::Term;
    use std::ops::Range;
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
            let hovered = list.first().map(|v| v.clone());
            Self {
                list,
                nth_window: Default::default(),
                cursor_range: Default::default(),
                cursor: Default::default(),
                hovered,
            }
        }
    }

    impl<T> Paginate<T>
    where
        T: Clone,
    {
        pub fn next(&mut self) -> bool {
            self.set_cursor_range();

            if self.cursor >= self.cursor_range.end {
                self.nth_window = self
                    .nth_window
                    .saturating_add(1)
                    .min(self.list.windows(self.get_limit()).count() - 1);
            } else {
                self.cursor = (self.cursor + 1).min(self.get_limit() - 1);
            }
            self.hovered = self.window().get(self.cursor).map(|v| v.clone());
            true
        }

        pub fn prev(&mut self) -> bool {
            self.set_cursor_range();

            if self.cursor < self.cursor_range.start {
                self.nth_window = self.nth_window.saturating_sub(1);
            } else {
                self.cursor = self.cursor.saturating_sub(1);
            }

            self.hovered = self.window().get(self.cursor).map(|v| v.clone());
            true
        }

        fn set_cursor_range(&mut self) {
            if self.nth_window == 0 {
                self.cursor_range = 0..self.get_limit() - 3;
            } else if self.nth_window == (self.list.windows(self.get_limit()).count() - 1) {
                self.cursor_range = 3..self.get_limit();
            } else {
                self.cursor_range = 3..self.get_limit() - 3;
            }
        }

        fn get_limit(&self) -> usize {
            (Term::size().rows - HELP_MARGIN - TOP_MARGIN).into()
        }

        pub fn window(&self) -> &[T] {
            self.list
                .windows(self.get_limit())
                .nth(self.nth_window)
                .unwrap()
        }
        pub fn hovered(&self) -> Option<&T> {
            self.hovered.as_ref()
        }
    }
}
