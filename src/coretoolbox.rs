use structopt::StructOpt;
use std::process::{Command, Stdio};
use directories;
use failure::{Fallible, bail};
use lazy_static::lazy_static;

lazy_static! {
    static ref APPDIRS : directories::ProjectDirs = directories::ProjectDirs::from("com", "coreos", "toolbox").expect("creating appdirs");
}

static PRESERVED_ENV : &[&str] = &["COLORTERM", 
        "DBUS_SESSION_BUS_ADDRESS",
        "DESKTOP_SESSION",
        "DISPLAY",
        "LANG",
        "SHELL",
        "SSH_AUTH_SOCK",
        "TERM",
        "VTE_VERSION",
        "XDG_CURRENT_DESKTOP",
        "XDG_DATA_DIRS",
        "XDG_MENU_PREFIX",
        "XDG_RUNTIME_DIR",
        "XDG_SEAT",
        "XDG_SESSION_DESKTOP",
        "XDG_SESSION_ID",
        "XDG_SESSION_TYPE",
        "XDG_VTNR",
];

static DEFAULT_IMAGE : &str = "registry.fedoraproject.org/f30/fedora-toolbox:30";

#[derive(Debug, StructOpt)]
#[structopt(name = "coretoolbox", about = "Toolbox")]
#[structopt(rename_all = "kebab-case")]
/// Main options struct
struct Opt {
}

fn podman() -> Command {
    Command::new("podman")
}

fn ensure_image(name: &str) -> Fallible<()> {
    if !podman().args(&["inspect", name]).stdout(Stdio::null()).status()?.success() {
        if !podman().args(&["pull", name]).status()?.success() {
            bail!("Failed to pull image");
        }
    }
    Ok(())
}

fn run() -> Fallible<()> {
    ensure_image(DEFAULT_IMAGE)?;
    Ok(())
}

/// Primary entrypoint
fn main() -> Fallible<()> {
    let opts = Opt::from_args();
    run()?;
    Ok(())
}
