use std::{path::Path, process::Stdio};

use tokio::process::Command;


pub async fn verify_minisign_signature<P: AsRef<Path>>(
    public_keys: &[P],
    signature_file: P,
    target_file: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("minisign");

    for key in public_keys {
        cmd.arg("-p").arg(key.as_ref());
    }

    cmd.arg("-Vm")
        .arg(target_file.as_ref())
        .arg("-x")
        .arg(signature_file.as_ref());

    cmd.stdout(Stdio::null());
    cmd.stdin(Stdio::null());

    let status = cmd.status().await?;

    if !status.success() {
        return Err("Minisign signature verification failed".into());
    }

    Ok(())
}


