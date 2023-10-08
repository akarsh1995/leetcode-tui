pub mod event;
pub mod step;
pub use event::Event;
use std::error::Error;
pub mod content;
pub mod errors;
pub mod popup;
pub mod utils;

pub type UBStrSender = tokio::sync::mpsc::UnboundedSender<Option<String>>;

pub mod input {
    use crate::SendError;

    #[derive(Default)]
    pub struct Input {
        pub visible: bool,
        current_text: Option<String>,
        sender: Option<super::UBStrSender>,
    }

    impl Input {
        pub fn text(&self) -> Option<&String> {
            self.current_text.as_ref()
        }
    }

    impl Input {
        pub fn close(&mut self) -> bool {
            self.current_text = None;
            if let Some(sender) = self.sender.take() {
                tokio::spawn(async move {
                    let _ = sender.send(None).emit_if_error();
                });
            }
            self.toggle()
        }

        pub fn char(&mut self, c: char) -> bool {
            if let Some(_text) = self.current_text.as_mut() {
                _text.push(c);
            } else {
                self.current_text = Some(c.into());
            }
            self.try_send();
            true
        }

        pub fn remove_char(&mut self) -> bool {
            if let Some(_text) = self.current_text.as_mut() {
                if !_text.is_empty() {
                    _text.pop();
                    self.try_send();
                }
            }
            true
        }

        pub fn try_send(&mut self) {
            let text = self.current_text.clone();
            if let Some(sender) = self.sender.clone() {
                tokio::spawn(async move {
                    let _ = sender.send(text).emit_if_error();
                });
            }
        }

        pub fn toggle(&mut self) -> bool {
            self.visible = !self.visible;
            true
        }

        pub fn reset_with(&mut self, sender: super::UBStrSender, default_input: Option<String>) {
            self.sender = Some(sender);
            self.current_text = default_input;
        }
    }
}

pub trait SendError<T, E> {
    fn emit_if_error(self) -> Result<T, E>;
}

impl<T, E> SendError<T, E> for Result<T, E>
where
    E: Error + Sized,
{
    fn emit_if_error(self) -> Result<T, E> {
        match self {
            Err(e) => {
                emit!(Error(e.to_string()));
                Err(e)
            }
            ok => ok,
        }
    }
}

pub fn init() {
    content::question::init();
}
