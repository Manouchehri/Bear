/*  Copyright (C) 2012-2018 by László Nagy
    This file is part of Bear.

    Bear is a tool to generate compilation database for clang tooling.

    Bear is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Bear is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */


#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate scopeguard;
#[macro_use]
extern crate serde_derive;

pub mod intercept;
pub mod semantic;
pub mod io;
pub mod command;

mod error {
    error_chain! {
        links {
            Intercept(crate::intercept::Error, crate::intercept::ErrorKind);
            Semantic(crate::semantic::Error, crate::semantic::ErrorKind);
            Clang(crate::io::clang::Error, crate::io::clang::ErrorKind);
        }
    }
}

pub use error::{Error, ErrorKind, Result, ResultExt};
