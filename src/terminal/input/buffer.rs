use std::mem;

use datatypes::{BufferSettings, EchoSettings, Key};
use datatypes::Key::*;

#[derive(Default)]
pub struct InputBuffer {
    data: String,
    cursor: usize,
}

impl InputBuffer {

    pub fn write(&mut self, key: &Key, buf: BufferSettings, echo: EchoSettings) -> Option<String> {
        match (self.cursor == self.data.len(), key) {
            (_, &Char(c)) if c == '\n' || buf.eol(c)    => {
                self.data.push(c);
                self.cursor = 0;
                Some(mem::replace(&mut self.data, String::new()))
            }
            (_, &Enter)                                 => {
                self.data.push('\n');
                self.cursor = 0;
                Some(mem::replace(&mut self.data, String::new()))
            }
            (_, &Char(c)) if buf.signal(c)              => Some(c.to_string()),
            (_, &Char(c)) if c == echo.lerase as char   => {
                self.data.clear();
                self.cursor = 0;
                None
            }
            (_, &Char(c)) if c == echo.lnext as char    => unimplemented!(),
            (_, &Char(c)) if c == echo.werase as char   => unimplemented!(),
            (true, &Char(c))                            => {
                self.data.push(c);
                self.cursor += 1;
                None
            }
            (false, &Char(c))                           => {
                self.data.remove(self.cursor);
                self.data.insert(self.cursor, c);
                self.cursor += 1;
                None
            }
            (true, &Backspace)                          => {
                self.data.pop();
                self.cursor -= 1;
                None
            }
            (false, &Backspace)                         => {
                self.cursor -= 1;
                self.data.remove(self.cursor);
                None
            }
            (false, &Delete)                            => {
                self.data.remove(self.cursor);
                self.cursor -= 1;
                None
            }
            (_, &LeftArrow) if self.cursor != 0         => { self.cursor -= 1; None }
            (false, &RightArrow)                        => { self.cursor += 1; None }
            (_, &Home)                                  => { self.cursor = 0; None }
            _                                           => None
        } 
    }

}
