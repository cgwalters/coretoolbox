use structopt::StructOpt;
use std::path::Path;
use std::process::{Command, Stdio};
use directories;
use failure::{Fallible, bail};
use lazy_static::lazy_static;

lazy_static! {
    static ref APPDIRS : directories::ProjectDirs = directories::ProjectDirs::from("com", "coreos", "toolbox").expect("creating appdirs");
}

static MAX_UID_COUNT : u32 = 65536;

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

#[derive(Debug, StructOpt)]
#[structopt(name = "coretoolbox", about = "Toolbox")]
#[structopt(rename_all = "kebab-case")]
/// Main options struct
struct Opt {
    #[structopt(short = "I", long = "image", default_value = "registry.fedoraproject.org/f30/fedora-toolbox:30")]
    /// Use a versioned installer binary
    image: String,

    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Cmd {
    Entrypoint,
}

fn cmd_podman() -> Command {
    if let Some(podman) = std::env::var_os("podman") {
        Command::new(podman)
    } else {
        Command::new("podman")
    }
}

/// Returns true if the host is OSTree based
fn is_ostree_based_host() -> bool {
    std::path::Path::new("/run/ostree-booted").exists()
}

/// Pull a container image if not present
fn ensure_image(name: &str) -> Fallible<()> {
    if !cmd_podman().args(&["inspect", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?.success() {
        if !cmd_podman().args(&["pull", name]).status()?.success() {
            bail!("Failed to pull image");
        }
    }
    Ok(())
}

fn getenv_required_utf8(n: &str) -> Result<String> {
    if let Some(v) = std::env::var_os(n) {
        v.to_str().ok_or_else(|| failure::format_err!("{} is invalid UTF-8", n))?.to_string()
    } else {
        bail!("{} is unset", n)
    }
}

fn run(opts: Opt) -> Fallible<()> {
    ensure_image(&opts.image)?;

    // Copy our own binary to the runtime dir so we can
    // exec ourself as the entrypoint.  In the future this
    // would be better with podman fd passing.
    let xdg_runtime_dir = getenv_required_utf8("XDG_RUNTIME_DIR")?;
    let xdg_self_path = format!("{}/toolbox.entrypoint", xdg_runtime_dir);
    std::fs::copy("/proc/self/exe", xdg_self_path)?;

    let mut podman = cmd_podman();
    podman.args(&["run", "--rm", "-ti", "--hostname", "toolbox",
                  "--name", "coreos-toolbox", "--network", "host",
                  "--privileged", "--security-opt", "label-disable"]);
    let real_uid : u32 = nix::unistd::getuid().into();
    let uid_plus_one = real_uid + 1;             
    let max_minus_uid = MAX_UID_COUNT - real_uid;     
    podman.args(&[format!("--uidmap={}:0:1", real_uid),
                  format!("--uidmap=0:1:{}", real_uid),
                  format!("--uidmap={}:{}:{}", uid_plus_one, uid_plus_one, max_minus_uid)]);
    // TODO: Detect what devices are accessible
    for p in &["/dev/bus", "/dev/dri", "/dev/fuse"] {
        if Path::new(p).exists() {
            podman.arg(format!("--volume={}:{}:rslave", p, p));
        }
    }
    for p in &["/usr", "/var", "/etc", "/run"] {
        podman.arg(format!("--volume={}:/host{}:rslave", p, p));
    }    
    if is_ostree_based_host() {
        podman.arg(format!("--volume=/sysroot:/host/sysroot:rslave"));
    } else {
        for p in &["/media", "/mnt", "/home", "/srv"] {
            podman.arg(format!("--volume={}:/host{}:rslave", p, p));
        }           
    }
    for n in PRESERVED_ENV.iter() {
        let v = match std::env::var_os(n) {
            Some(v) => v,
            None => continue, 
        };
        let v = v.to_str().ok_or_else(|| failure::format_err!("{} contains invalid UTF-8", n))?;
        podman.arg(format!("--env={}={}", n, v));
    }
    podman.arg("--entrypoint={}", xdg_self_path)
    podman.arg(opts.image);
    eprintln!("running {:?}", podman);
    if !podman.status()?.success() {
        bail!("podman failed");
    }
    Ok(())
}

/// Primary entrypoint
fn main() -> Fallible<()> {
    let opts = Opt::from_args();
    if let Some(cmd) = opts.cmd.as_ref() {
        match cmd {
            Cmd::Entrypoint => {
                println!("entrypoint");
            }
        }
    } else {
        run(opts)?;
    }
    Ok(())
}
