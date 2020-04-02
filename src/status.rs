// This file is part of wiiunhide.
// Copyright (C) 2020  bsdf

// wiiunhide is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// wiiunhide is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with wiiunhide.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt;
use std::fmt::{Display, Formatter};

pub enum Status {
    Hidden,
    Visible,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Status::Hidden => write!(f, "HIDDEN"),
            Status::Visible => write!(f, "VISIBLE"),
        }
    }
}
