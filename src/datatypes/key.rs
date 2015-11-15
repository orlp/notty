//  notty is a new kind of terminal emulator.
//  Copyright (C) 2015 without boats
//  
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//  
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Affero General Public License for more details.
//  
//  You should have received a copy of the GNU Affero General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
use std::borrow::Cow;

use self::Key::*;

/// Mostly, these represent keys on the keyboard. Boolean fields are true for key presses and
/// false for key releases.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    /// Any key which generates a unicode code point when pressed.
    Char(char),
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Enter,
    ShiftLeft,
    ShiftRight,
    CtrlLeft,
    CtrlRight,
    AltLeft,
    AltGr,
    Meta,
    Menu,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    Delete,
    CapsLock,
    NumLock,
    ScrollLock,
    /// A function; the byte value indicates the key number.
    Function(u8),
    /// This key is not generated by the keyboard but as a response to some escape code sent
    /// to the output.
    Cmd(Cow<'static, str>),
    /// A selection from an in-terminal drop down menu.
    MenuSelection(usize),
}


impl Key {
    pub fn is_modifier(&self) -> bool {
        match *self {
            ShiftLeft | ShiftRight | CtrlLeft | CtrlRight | AltLeft | AltGr | CapsLock => true,
            _   => false
        }
    }
}
