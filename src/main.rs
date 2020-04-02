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

mod backup;
mod mbr;
mod status;
mod util;

use status::Status;
use std::io;
use std::io::{Error, ErrorKind};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    version = "0.1",
    about = "hide/unhide your hd to get rid of the wiiu nag screen by modifying the MBR signature. USE AT YOUR OWN RISK"
)]
struct Opts {
    /// block device of the drive. you must have read/write privileges for this device
    device: String,

    /// specify filename for MBR backup [default: mbr.bin]
    #[structopt(short = "b", long)]
    backup_file: Option<String>,

    /// don't backup your MBR (NOT RECOMMENDED!!)
    #[structopt(long)]
    no_backup: bool,

    /// don't ask for confirmation before executing the operation
    #[structopt(short = "y", long)]
    assume_yes: bool,

    /// restore backup MBR. should be used as a last resort
    #[structopt(short = "R", long, name = "backup file")]
    restore: Option<String>,

    /// display drive status and exit. no modifications will be executed
    #[structopt(short = "c", long)]
    check: bool,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::from_args();

    println!("wiiunhide 0.1\n");

    if let Some(filename) = &opts.restore {
        backup::restore_backup(&opts.device, &filename[..])?;
        return Ok(());
    }

    mbr::read_mbr(&opts.device).and_then(|mbr| match mbr::check_mbr(mbr) {
        Some(status) => {
            if opts.check {
                println!("{} is currently {}", &opts.device, status);
                return Ok(());
            }

            println!(
                "{} is currently {}. it will be modified to be {}.",
                &opts.device,
                status,
                match status {
                    Status::Visible => Status::Hidden,
                    Status::Hidden => Status::Visible,
                }
            );

            if !opts.assume_yes && !util::confirm("do you wish to continue? [y/N]: ") {
                println!("aborting operation.");
                return Ok(());
            }

            if !opts.no_backup {
                let filename = opts.backup_file.unwrap_or("mbr.bin".to_string());
                backup::write_backup(&filename, &mbr[..])?;
            }

            mbr::toggle_mbr(&opts.device, status)
        }

        None => Err(Error::new(
            ErrorKind::InvalidData,
            "MBR not found on device.",
        )),
    })
}
