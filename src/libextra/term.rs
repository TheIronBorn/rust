// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Simple ANSI color library

#[allow(missing_doc)];

use core::prelude::*;

use core::io;
use core::os;

use terminfo::*;
use terminfo::searcher::open;
use terminfo::parser::compiled::parse;
use terminfo::parm::{expand, Number};

// FIXME (#2807): Windows support.

pub static color_black: u8 = 0u8;
pub static color_red: u8 = 1u8;
pub static color_green: u8 = 2u8;
pub static color_yellow: u8 = 3u8;
pub static color_blue: u8 = 4u8;
pub static color_magenta: u8 = 5u8;
pub static color_cyan: u8 = 6u8;
pub static color_light_gray: u8 = 7u8;
pub static color_light_grey: u8 = 7u8;
pub static color_dark_gray: u8 = 8u8;
pub static color_dark_grey: u8 = 8u8;
pub static color_bright_red: u8 = 9u8;
pub static color_bright_green: u8 = 10u8;
pub static color_bright_yellow: u8 = 11u8;
pub static color_bright_blue: u8 = 12u8;
pub static color_bright_magenta: u8 = 13u8;
pub static color_bright_cyan: u8 = 14u8;
pub static color_bright_white: u8 = 15u8;

pub fn esc(writer: @io::Writer) { writer.write([0x1bu8, '[' as u8]); }

pub struct Terminal {
    color_supported: bool,
    priv out: @io::Writer,
    priv ti: ~TermInfo
}

pub impl Terminal {
    pub fn new(out: @io::Writer) -> Result<Terminal, ~str> {
        let term = os::getenv("TERM");
        if term.is_none() {
            return Err(~"TERM environment variable undefined");
        }

        let entry = open(term.unwrap());
        if entry.is_err() {
            return Err(entry.get_err());
        }

        let ti = parse(entry.get(), false);
        if ti.is_err() {
            return Err(entry.get_err());
        }

        let mut inf = ti.get();
        let cs = *inf.numbers.find_or_insert(~"colors", 0) >= 16 && inf.strings.find(&~"setaf").is_some()
            && inf.strings.find(&~"setab").is_some();

        return Ok(Terminal {out: out, ti: inf, color_supported: cs});
    }
    fn fg(&self, color: u8) {
        self.out.write(expand(*self.ti.strings.find(&~"setaf").unwrap(), [Number(color as int)], [], []));
    }
    fn bg(&self, color: u8) {
        self.out.write(expand(*self.ti.strings.find(&~"setab").unwrap(), [Number(color as int)], [], []));
    }
    fn reset(&self) {
        self.out.write(expand(*self.ti.strings.find(&~"op").unwrap(), [], [], []));
    }
}
