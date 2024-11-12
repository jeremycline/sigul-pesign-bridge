// SPDX-License-Identifier: MIT
// Copyright (c) Microsoft Corporation.
//
// Tests the integration with pesign-client.
//
// These tests require that the `pesign-client` binary be in your PATH.
// Additionally, since pesign-client doesn't offer a way to configure
// the socket it connects to, the user running the test must be able to
// read/write to `/run/pesign/socket`.

use std::{
    process::{Child, Command, Stdio},
    sync::Once,
    time::Duration,
};

use anyhow::{anyhow, Result};
use assert_cmd::cargo::CommandCargoExt;
use nix::sys::{
    signal::{self, Signal},
    stat::Mode,
};

static UMASK: Once = Once::new();

fn start_service() -> Result<Child> {
    UMASK.call_once(|| {
        let mut umask = Mode::empty();
        umask.insert(Mode::S_IRWXO);
        nix::sys::stat::umask(umask);
    });

    let socket_path = "/run/pesign/socket";
    if let Ok(_metadata) = std::fs::metadata(socket_path) {
        return Err(anyhow!(
            "{socket_path} exists; unable to start test instance"
        ));
    };
    let mut command = Command::cargo_bin("sigul-pesign-bridge")?;
    command
        .env("SIGUL_PESIGN_LOG", "trace")
        .arg(format!("--socket={socket_path}"))
        .arg("listen")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());
    let child = command.spawn()?;

    let mut tries = 0;
    while let Err(error) = std::fs::metadata(socket_path) {
        std::thread::sleep(Duration::from_millis(5));
        tries += 1;
        if tries > 100 {
            return Err(anyhow!("Faild to start service: {error:?}"));
        }
    }

    Ok(child)
}

// Test that the daemon implements the "is-unlocked" command.
#[test]
fn is_unlocked() -> Result<()> {
    let service = start_service()?;
    let client_output = Command::new("pesign-client")
        .arg("--is-unlocked")
        .arg("--token=Test Cert DB")
        .output()
        .unwrap();

    signal::kill(
        nix::unistd::Pid::from_raw(service.id().try_into()?),
        Signal::SIGTERM,
    )?;
    let service_output = service.wait_with_output()?;

    assert!(client_output.status.success());
    assert_eq!(
        "token \"Test Cert DB\" is unlocked\n",
        String::from_utf8_lossy(&client_output.stdout)
    );
    assert!(service_output.status.success());

    Ok(())
}
