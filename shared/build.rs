// SPDX-License-Identifier: WTFPL
//! shared binary build script.
//!
//! right now this creates the certificate shit for webtransport.

use core::error::Error;
use std::{fs, path::PathBuf};

use wtransport::{
    Identity,
    tls::{Certificate, Sha256DigestFmt},
};

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .ok_or("shared/ must live inside the workspace root")?
        .to_path_buf();

    let identity = Identity::self_signed(["localhost", "127.0.0.1", "::1"])?;
    let certificate = &identity.certificate_chain().as_slice()[0];

    let fingerprint = certificate
        .hash()
        .fmt(Sha256DigestFmt::DottedHex)
        .replace(':', "")
        .to_ascii_uppercase();

    let certificate_pem = identity
        .certificate_chain()
        .as_slice()
        .iter()
        .map(Certificate::to_pem)
        .collect::<String>();

    let private_key_pem = identity.private_key().to_secret_pem();

    write_certs(
        &workspace_root.join("shared/certs"),
        &certificate_pem,
        &private_key_pem,
        &fingerprint,
        true,
    )?;

    write_certs(
        &workspace_root.join("server/certs"),
        &certificate_pem,
        &private_key_pem,
        &fingerprint,
        true,
    )?;

    write_certs(
        &workspace_root.join("client/certs"),
        &certificate_pem,
        &private_key_pem,
        &fingerprint,
        false,
    )?;

    println!("cargo:warning=WebTransport cert fingerprint: {fingerprint}");

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

fn write_certs(
    dir: &PathBuf,
    cert_pem: &str,
    key_pem: &str,
    fingerprint: &str,
    include_key: bool,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(dir)?;
    fs::write(dir.join("cert.pem"), cert_pem)?;
    if include_key {
        fs::write(dir.join("key.pem"), key_pem)?;
    }
    fs::write(dir.join("digest.txt"), fingerprint)?;
    Ok(())
}
