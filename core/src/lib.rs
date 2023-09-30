pub mod event;
pub mod step;
pub mod topic;
pub use event::Event;

pub mod question {
    use leetcode_db::{Db, DbQuestion, DbTopic};
    use shared::log;

    use crate::{emit, utils::Paginate};

    pub struct Questions {
        paginate: Paginate<DbQuestion>,
    }

    impl Questions {
        pub fn new() -> Self {
            Self {
                paginate: Paginate::new(vec![]),
            }
        }
    }

    impl Questions {
        pub fn prev(&mut self) -> bool {
            self.paginate.prev()
        }

        pub fn next(&mut self) -> bool {
            self.paginate.next()
        }

        pub fn window(&self) -> &[DbQuestion] {
            self.paginate.window()
        }

        pub fn hovered(&self) -> Option<&DbQuestion> {
            self.paginate.hovered()
        }
    }

    impl Questions {
        pub fn get_questions_by_topic(&mut self, topic: DbTopic, db: Db) {
            tokio::spawn(async move {
                let questions = topic.fetch_questions(&db.clone()).await;
                match questions {
                    Ok(_questions) => {
                        emit!(Questions(_questions));
                        emit!(Render);
                    }
                    Err(e) => log::error!("Problem fetching questions for topic {topic:?}: {e}"),
                }
            });
        }

        pub fn set_questions(&mut self, questions: Vec<DbQuestion>) {
            self.paginate.update_list(questions)
        }
    }
}

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

        pub fn update_list(&mut self, list: Vec<T>) {
            *self = Self::new(list)
        }
    }

    impl<T> Paginate<T>
    where
        T: Clone,
    {
        pub fn next(&mut self) -> bool {
            self.set_cursor_range();
            let old_cursor = self.cursor;
            let old_window = self.nth_window;

            if self.cursor >= self.cursor_range.end {
                self.nth_window = self
                    .nth_window
                    .saturating_add(1)
                    .min(self.list.windows(self.get_limit()).count() - 1);
            } else {
                self.cursor = (self.cursor + 1).min(self.get_limit() - 1);
            }
            self.hovered = self.window().get(self.cursor).map(|v| v.clone());
            self.cursor != old_cursor || self.nth_window != old_window
        }

        pub fn prev(&mut self) -> bool {
            self.set_cursor_range();
            let old_cursor = self.cursor;
            let old_window = self.nth_window;

            if self.cursor < self.cursor_range.start {
                self.nth_window = self.nth_window.saturating_sub(1);
            } else {
                self.cursor = self.cursor.saturating_sub(1);
            }

            self.hovered = self.window().get(self.cursor).map(|v| v.clone());
            self.cursor != old_cursor || self.nth_window != old_window
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
            ((Term::size().rows - HELP_MARGIN - TOP_MARGIN) as usize).min(self.list.len())
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
