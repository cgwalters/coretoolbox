use failure::{bail, Fallible};
use serde::Deserialize;
use serde_json;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use varlink::{Call, Connection, VarlinkService};

use crate::io_podman::*;

#[allow(dead_code)]
pub(crate) enum InspectType {
    Container,
    Image,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct ImageInspect {
    pub id: String,
    pub names: Option<Vec<String>>,
}

pub(crate) fn client() -> Fallible<VarlinkClient> {
    let podman = std::env::var_os("podman").map(|s| s.to_str().unwrap().to_string()).unwrap_or_else(|| "podman".into());
    let cmd = format!("{} varlink $VARLINK_ADDRESS", podman);
    let conn = Connection::with_activate(cmd.as_str())?;
    Ok(VarlinkClient::new(conn.clone()))
}

pub(crate) fn cmd() -> Command {
    if let Some(podman) = std::env::var_os("podman") {
        Command::new(podman)
    } else {
        Command::new("podman")
    }
}

/// Returns true if an image or container is in the podman
/// storage.
pub(crate) fn has_object(t: InspectType, name: &str) -> Fallible<bool> {
    let mut iface = client()?;
    match t {
        InspectType::Container => {
            match iface.get_container(name.to_string()).call() {
                Ok(_) => Ok(true),
                Err(e) => {
                    match e.kind() {
                        ErrorKind::ContainerNotFound(_) => Ok(false),
                        _ => bail!(e.to_string())
                    }
                },
            }
        },
        InspectType::Image => {
            match iface.get_image(name.to_string()).call() {
                Ok(_) => Ok(true),
                Err(e) => {
                    match e.kind() {
                        ErrorKind::ImageNotFound(_) => Ok(false),
                        _ => bail!(e.to_string())
                    }
                },
            }
        }
    }
}

pub(crate) fn image_inspect<I, S>(args: I) -> Fallible<Vec<ImageInspect>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut proc = cmd()
        .stdout(Stdio::piped())
        .args(&["images", "--format", "json"])
        .args(args)
        .spawn()?;
    let sout = proc.stdout.take().expect("stdout piped");
    let mut sout = std::io::BufReader::new(sout);
    let res = if sout.fill_buf()?.len() > 0 {
        serde_json::from_reader(sout)?
    } else {
        Vec::new()
    };
    if !proc.wait()?.success() {
        bail!("podman images failed")
    }
    Ok(res)
}

pub(crate) fn test_varlink() -> Fallible<()> {
    let mut iface = client()?;
    dbg!(iface.get_version().call().map_err(|e| failure::err_msg(e.to_string()))?);
    Ok(())
}