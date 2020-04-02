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

use std::io;
use std::io::prelude::*;

pub fn confirm(msg: &str) -> bool {
    let mut yn = String::with_capacity(1);
    print!("{}", msg);

    io::stdout()
        .flush()
        .and_then(|_| io::stdin().read_line(&mut yn))
        .map(|_| match &yn.trim().to_lowercase()[..] {
            "y" => true,
            _ => false,
        })
        .unwrap_or(false)
}
