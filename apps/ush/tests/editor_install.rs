use std::{fs, process::Command};
#[test]
fn offline_package_is_a_valid_versioned_vsix() {
    let dir = tempfile::tempdir().unwrap();
    let vsix = dir.path().join("language pack.vsix");
    let out = Command::new(env!("CARGO_BIN_EXE_ush"))
        .args(["editor", "install", "cursor", "--output"])
        .arg(&vsix)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let data = fs::read(&vsix).unwrap();
    assert!(data.starts_with(b"PK\x03\x04"));
    // Use an independent ZIP implementation, validating CRCs and manifest contents.
    let check=Command::new("python3").arg("-c").arg("import sys,zipfile,json; z=zipfile.ZipFile(sys.argv[1]); assert z.testzip() is None; p=json.loads(z.read('extension/package.json')); assert p['version']==sys.argv[2]; assert p['contributes']['languages'][0]['extensions']==['.ush']; assert len(z.read('extension/dist/extension.js'))>1000").arg(vsix).arg(env!("CARGO_PKG_VERSION")).status().unwrap();
    assert!(check.success());
}
#[cfg(unix)]
#[test]
fn installer_passes_vsix_as_one_argument_and_preserves_editor_failure() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let cli = dir.path().join("editor command");
    fs::write(
        &cli,
        "#!/bin/sh\n[ \"$1\" = --install-extension ] || exit 2\n[ -f \"$2\" ] || exit 3\nexit 42\n",
    )
    .unwrap();
    fs::set_permissions(&cli, fs::Permissions::from_mode(0o755)).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_ush"))
        .args(["editor", "install", "vscode", "--cli"])
        .arg(cli)
        .env("RUST_BACKTRACE", "1")
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("exit 42"), "{stderr}");
    assert!(!stderr.contains("stack backtrace"), "{stderr}");
}
