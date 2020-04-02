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

use crate::{mbr, util};

use std::fs::{File, OpenOptions};
use std::io;
use std::io::{prelude::*, SeekFrom};

pub fn write_backup(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(bytes)?;

    Ok(())
}

pub fn restore_backup(device: &str, filename: &str) -> io::Result<()> {
    mbr::read_mbr(filename).and_then(|mbr| {
        println!("this operation will attempt to restore an MBR backup made by this program.");
        if util::confirm("do you wish to continue? [y/N]: ") {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(false)
                .create_new(false)
                .open(device)?;
            file.seek(SeekFrom::Start(0))?;
            file.write_all(&mbr[..])
        } else {
            println!("aborting operation.");
            Ok(())
        }
    })
}
