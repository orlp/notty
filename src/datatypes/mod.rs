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
//! The datatypes module defines the abstract datatypes used by other components of notty.
//! 
//! The types in this module are intended to be passed between modules. As a design restriction,
//! any methods on any type in this submodule are required to take the receiver immutably.
use std::borrow::Cow;

use image::DynamicImage;

mod iter;
mod movement;
mod region;

pub use self::iter::CoordsIter;
pub use self::movement::Movement;
pub use self::region::Region;

pub mod args {
    pub use super::{
        Area,
        Coords,
        Color,
        Direction,
        InputMode,
        MediaAlignment,
        MediaPosition,
        Movement,
        Region,
        Style,
    };
    pub use super::Area::*;
    pub use super::Direction::*;
    pub use super::InputMode::*;
    pub use super::MediaAlignment::*;
    pub use super::MediaPosition::*;
    pub use super::Movement::*;
    pub use super::Style::*;
}

/// An abstractly defined section of the grid.
///
/// Areas can be defined in terms of the current cursor position and the bounds of the grid. They
/// are converted into concrete sections of the screen when commands using Areas are applied.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Area {
    /// The cell the cursor is in.
    CursorCell,
    /// The row the cursor is in.
    CursorRow,
    /// The column the cursor is in.
    CursorColumn,
    /// All cells the cursor would traverse through in performing a movement (including the cell
    /// the cursor is in now, and the cell it would end in).
    CursorTo(Movement),
    /// The rectangle bound in one corner by the cursor position and another by this coordinate.
    CursorBound(Coords),
    /// The entire screen.
    WholeScreen,
    /// A concrete rectangular section of the screen.
    Bound(Region),
    /// The rows between the two parameters, inclusive of the first but not the second.
    Rows(u32, u32),
    /// The columns between the two parameters, inclusive of the first but not the second.
    Columns(u32, u32),
    /// Everything below the row the cursor is in, the boolean determines if this is inclusive of
    /// the cursor or not (inclusive = true).
    BelowCursor(bool),
}

/// Data that could be placed in a character cell.
#[derive(Clone)]
pub enum CellData {
    /// A single unicode code point.
    Char(char), 
    /// An extension code point such as U+301. Normally, writing this to the screen does not
    /// overwrite a cell, but instead applies it to the char in the cell.
    ExtensionChar(char),
    /// A multi code-point grapheme, such as a Hangul triplet.
    Grapheme(String),
    /// Non-character media data, with a mime type, positioning and size info.
    Image {
        pos: MediaPosition,
        width: u32,
        height: u32,
        data: DynamicImage,
    }
}

/// A kind of escape code format (used for structuring response strings).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Code {
    ANSI,
    Notty,
}

/// A 24-bit rgb color sequence.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

/// A corodinate pair.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Coords {
    pub x: u32,
    pub y: u32,
}

/// A direction of movement across the grid.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rev(&self) -> Direction {
        match *self {
            Direction::Up       => Direction::Down,
            Direction::Down     => Direction::Up,
            Direction::Left     => Direction::Right,
            Direction::Right    => Direction::Left,
        }
    }
}

/// The mode the input processor is in.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InputMode {
    /// ANSI-compatible mode.
    Ansi,
    /// ANSI-compatible mode with application arrow key input.
    Application,
    Extended,
}

/// Mostly, these represent keys on the keyboard. Boolean fields are true for key presses and
/// false for key releases.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    /// Any key which generates a unicode code point when pressed.
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Enter,
    ShiftLeft,
    ShiftRight,
    CtrlLeft,
    CtrlRight,
    AltLeft,
    AltRight,
    MetaLeft,
    MetaRight,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MediaAlignment {
    LeftTop, Center, RightBottom
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MediaPosition {
    Display(MediaAlignment, MediaAlignment),
    Fill,
    Fit,
    Stretch,
    Tile
}

impl Default for MediaPosition {
    fn default() -> MediaPosition {
        MediaPosition::Display(MediaAlignment::LeftTop, MediaAlignment::LeftTop)
    }
}

/// Set rich text styles. Booleans represent on or off.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Style {
    /// Field is number of underlines (between 0 and 2).
    Underline(u8),
    Bold(bool),
    Italic(bool),
    Blink(bool),
    InvertColors(bool),
    Strikethrough(bool),
    Opacity(u8),
    FgColor(Color),
    FgColorCfg(Option<u8>),
    BgColor(Color),
    BgColorCfg(Option<u8>),
}
