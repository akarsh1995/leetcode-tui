pub mod event;
pub mod step;
pub mod topic;
pub use event::Event;
use std::error::Error;
pub mod errors;
pub mod popup;
pub mod question;
pub mod utils;

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
    question::init();
}
