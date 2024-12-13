// SPDX-License-Identifier: MIT
// Copyright (c) Microsoft Corporation.

use std::path::PathBuf;

use clap::Parser;

/// A sigul client that offers the pesign daemon interface
///
/// Run a service that presents a Unix socket like the pesign command-line tool offers when it is
/// run as a daemon.
///
/// Rather than signing the PE file, however, this application will act as a sigul client and
/// forward it to a sigul signing server.
///
/// The Unix socket this service offers can be used by pesign-client.
#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Cli {
    /// The path to use for the unix socket the service creates
    #[arg(long, env = "SIGUL_PESIGN_BRIDGE_SOCKET", default_value_os_t = PathBuf::from("/run/sigul-pesign-bridge/service.sock"))]
    pub socket: PathBuf,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Command {
    /// Run the service.
    Listen {
        /// The path to a file containing the passphrase to unlock the sigul client certificate.
        ///
        /// If using systemd to manage the service, it will first look for any systemd credential
        /// loaded with LoadCredentialsEncrypted and the "sigul-passphrase" ID. If that is not found
        /// it will fall back to using this command-line argument to discover the secret.
        #[arg(long, env = "SIGUL_PASSPHRASE_PATH")]
        sigul_passphrase_path: Option<PathBuf>,
    },
    /// Query the status of the service.
    Status,
}
