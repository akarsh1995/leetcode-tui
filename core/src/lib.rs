pub mod event;
pub mod step;
pub mod topic;
pub use event::Event;
pub mod question;
pub mod utils;

pub mod popup {
    #[derive(Default)]
    pub struct Popup {
        pub visible: bool,
        lines: Vec<String>,
    }

    impl Popup {
        pub fn toggle(&mut self) -> bool {
            self.visible = !self.visible;
            true
        }

        pub fn get_text(&self) -> &Vec<String> {
            &self.lines
        }

        pub fn set_lines(&mut self, lines: Vec<String>) {
            self.lines = lines;
        }

        pub fn get_lines(&self) -> &Vec<String> {
            &self.lines
        }
    }
}
