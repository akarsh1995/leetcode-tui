pub mod event;
pub mod step;
pub use event::Event;
use std::error::Error;
pub mod content;
pub mod errors;
pub mod help;
pub mod input;
pub mod popup;
pub mod utils;

pub type UBStrSender = tokio::sync::mpsc::UnboundedSender<Option<String>>;

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
