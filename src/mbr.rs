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

use crate::status::Status;

use std::fs::{File, OpenOptions};
use std::io;
use std::io::{prelude::*, SeekFrom};

const MBR_SIZE: usize = 512;
const SIG_OFFSET: usize = 510;
const SIG_VISIBLE: &[u8] = &[0x55, 0xAA];
const SIG_HIDDEN: &[u8] = &[0x55, 0xAB];

pub fn read_mbr(device: &str) -> io::Result<[u8; MBR_SIZE]> {
    let mut file = File::open(device)?;
    let mut mbr = [0; MBR_SIZE];
    file.read(&mut mbr).map(|_| mbr)
}

pub fn check_mbr(mbr: [u8; MBR_SIZE]) -> Option<Status> {
    match &mbr[SIG_OFFSET..] {
        SIG_VISIBLE => Some(Status::Visible),
        SIG_HIDDEN => Some(Status::Hidden),
        _ => None,
    }
}

pub fn toggle_mbr(device: &str, current_status: Status) -> io::Result<()> {
    let sig = match current_status {
        Status::Visible => SIG_HIDDEN,
        Status::Hidden => SIG_VISIBLE,
    };

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(false)
        .create_new(false)
        .open(device)?;

    file.seek(SeekFrom::Start(SIG_OFFSET as u64))?;
    file.write_all(sig)?;

    println!("done.");
    Ok(())
}
