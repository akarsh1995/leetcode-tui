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
