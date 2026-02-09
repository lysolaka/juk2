use vte::{Params, Parser, Perform};

#[derive(defmt::Format, Copy, Clone, PartialEq, Eq)]
pub enum Key {
    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowLeft,
    Home,
    End,
    Backspace,
    Delete,
    CtrlBackspace,
    CtrlDelete,
    CtrlRight,
    CtrlLeft,
}

#[derive(defmt::Format, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Print(char),
    Execute(u8),
    KeyEvent(Key),
}

pub struct EventParser {
    parser: Parser<4>,
    performer: EventBuf,
}

impl EventParser {
    pub fn new() -> Self {
        Self {
            parser: Parser::new_with_size(),
            performer: EventBuf {
                event: None,
                terminated: false,
                sentinel_seen: false,
            },
        }
    }

    pub fn advance(&mut self, byte: u8) -> Option<Event> {
        self.parser.advance(&mut self.performer, &[byte]);

        self.performer.event.take()
    }

    pub fn terminated(&self) -> bool {
        self.performer.terminated
    }

    pub fn unterminate(&mut self) {
        self.performer.terminated = false;
        self.performer.sentinel_seen = false;
    }
}

struct EventBuf {
    event: Option<Event>,
    terminated: bool,
    sentinel_seen: bool,
}

fn get_param(params: &Params, index: usize) -> u16 {
    params
        .iter()
        .nth(index)
        .and_then(|p| p.get(0))
        .copied()
        .unwrap_or(0)
}

impl Perform for EventBuf {
    fn print(&mut self, c: char) {
        self.event = if c == '\u{7f}' {
            Some(Event::KeyEvent(Key::Backspace))
        } else {
            Some(Event::Print(c))
        };
    }

    fn execute(&mut self, byte: u8) {
        if byte == 0x00 {
            if self.sentinel_seen {
                self.terminated = true;
            } else {
                self.sentinel_seen = true;
            }
        } else {
            self.sentinel_seen = false;
        }

        self.event = if byte == 0x08 {
            Some(Event::KeyEvent(Key::CtrlBackspace))
        } else {
            Some(Event::Execute(byte))
        };
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, action: char) {
        if ignore || !intermediates.is_empty() {
            return;
        }

        let p0 = get_param(params, 0);
        let p1 = get_param(params, 1);

        let key = match (action, params.len(), p0, p1) {
            // ARROWS
            ('A', 1, 0, _) => Key::ArrowUp,
            ('B', 1, 0, _) => Key::ArrowDown,
            ('C', 1, 0, _) => Key::ArrowRight,
            ('D', 1, 0, _) => Key::ArrowLeft,
            // HOME / END
            ('H', 1, 0, _) => Key::Home,
            ('F', 1, 0, _) => Key::End,
            // DEL
            ('~', 1, 3, _) => Key::Delete,
            ('~', 2, 3, 5) => Key::CtrlDelete,
            // CTRL + ARROW
            ('C', 2, 1, 5) => Key::CtrlRight,
            ('D', 2, 1, 5) => Key::CtrlLeft,
            _ => return,
        };

        self.event = Some(Event::KeyEvent(key));
    }
}
