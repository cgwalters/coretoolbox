use structopt::StructOpt;
use directories;
use failure::Fallible;
use lazy_static::lazy_static;

lazy_static! {
    static ref APPDIRS : directories::ProjectDirs = directories::ProjectDirs::from("org", "openshift", "xokdinst").expect("creating appdirs");
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
enum Opt {
    /// Enter toolbox
    Enter,
}

fn podman() -> std::process::Command {
    let mut cmd = std::process::Command::new("podman");
}

fn ensure_image(name: &str) -> Fallible<()> {

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
