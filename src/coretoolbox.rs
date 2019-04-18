use std::{fs, io};
use std::io::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use structopt::StructOpt;
#[macro_use]
extern crate clap;
use directories;
#[macro_use]
extern crate failure;
use failure::Fallible;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

lazy_static! {
    static ref APPDIRS : directories::ProjectDirs = directories::ProjectDirs::from("org", "openshift", "xokdinst").expect("creating appdirs");
}

#[derive(Debug, StructOpt)]
#[structopt(name = "coretoolbox", about = "Toolbox")]
#[structopt(rename_all = "kebab-case")]
/// Main options struct
enum Opt {
    /// Enter toolbox
    Enter,
}

/// Primary entrypoint
fn main() -> Fallible<()> {
    match Opt::from_args() {
        Opt::Enter(o) => {
            enter()?;
        },
    }

    Ok(())
}
