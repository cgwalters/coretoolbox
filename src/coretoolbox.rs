use structopt::StructOpt;
use directories;
use failure::Fallible;
use lazy_static::lazy_static;

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

fn enter() -> Fallible<()> {
    Ok(())
}

/// Primary entrypoint
fn main() -> Fallible<()> {
    match Opt::from_args() {
        Opt::Enter => {
            enter()?;
        },
    }

    Ok(())
}
