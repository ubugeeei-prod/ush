//! Offline editor installation: the release binary embeds the complete extension.
mod archive;
use anyhow::{Context, Result, bail};
use clap::Subcommand;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Clone, Subcommand)]
pub enum Action {
    /// Install the bundled extension into VS Code, Cursor or VSCodium.
    Install {
        #[arg(default_value = "vscode", value_parser = ["vscode", "cursor", "vscodium"])]
        editor: String,
        /// Editor command, including an absolute path when it is not on PATH.
        #[arg(long)]
        cli: Option<PathBuf>,
        /// Write a VSIX package for offline/manual installation instead.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

pub fn run(action: &Action) -> Result<()> {
    let Action::Install {
        editor,
        cli,
        output,
    } = action;
    let executable = std::env::current_exe()?;
    let server = executable.with_file_name("ush_lsp");
    let server = if server.is_file() {
        server.to_string_lossy().into_owned()
    } else {
        "ush_lsp".into()
    };
    let bytes = package(&server)?;
    if let Some(path) = output {
        std::fs::write(path, bytes)
            .with_context(|| format!("could not write {}", path.display()))?;
        println!("Wrote {}", path.display());
        return Ok(());
    }
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("ush.vsix");
    std::fs::write(&path, bytes)?;
    let default = match editor.as_str() {
        "cursor" => "cursor",
        "vscodium" => "codium",
        _ => "code",
    };
    let command = cli.as_deref().unwrap_or_else(|| Path::new(default));
    let status = Command::new(command).arg("--install-extension").arg(&path).status()
        .with_context(|| format!("could not start {}: add the editor command to PATH, pass --cli /path/to/editor, or export a VSIX with --output ush.vsix", command.display()))?;
    if !status.success() {
        bail!(
            "{editor} could not install the ush extension (exit {}). See the editor output above",
            status.code().unwrap_or(1)
        );
    }
    println!("Installed ush for {editor}. Open a .ush file to start the language server.");
    Ok(())
}

fn package(server: &str) -> Result<Vec<u8>> {
    let mut manifest: serde_json::Value =
        serde_json::from_str(include_str!("../../../editors/vscode/package.json"))?;
    manifest["version"] = env!("CARGO_PKG_VERSION").into();
    manifest["contributes"]["configuration"]["properties"]["ush.server.path"]["default"] =
        server.into();
    // Build-time dependencies are not part of the installed extension.
    manifest.as_object_mut().unwrap().remove("devDependencies");
    manifest.as_object_mut().unwrap().remove("scripts");
    let package = serde_json::to_vec_pretty(&manifest)?;
    let xml = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011">
<Metadata><Identity Language="en-US" Id="ush" Version="{}" Publisher="ubugeeei"/><DisplayName>ush</DisplayName><Description xml:space="preserve">ush language support</Description><Tags>ush,shell</Tags><Categories>Programming Languages</Categories><GalleryFlags>Public</GalleryFlags><Properties><Property Id="Microsoft.VisualStudio.Code.Engine" Value="^1.90.0"/></Properties></Metadata>
<Installation><InstallationTarget Id="Microsoft.VisualStudio.Code"/></Installation><Dependencies/>
<Assets><Asset Type="Microsoft.VisualStudio.Code.Manifest" Path="extension/package.json" Addressable="true"/></Assets>
</PackageManifest>"#,
        env!("CARGO_PKG_VERSION")
    );
    Ok(archive::zip(&[
        ("extension.vsixmanifest", xml.as_bytes()),
        ("[Content_Types].xml", br#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="json" ContentType="application/json"/><Default Extension="js" ContentType="application/javascript"/><Default Extension="vsixmanifest" ContentType="text/xml"/><Default Extension="md" ContentType="text/markdown"/><Default Extension="txt" ContentType="text/plain"/></Types>"#),
        ("extension/package.json", &package),
        ("extension/dist/extension.js", include_bytes!("../../../editors/vscode/dist/extension.js")),
        ("extension/syntaxes/ush.tmLanguage.json", include_bytes!("../../../editors/vscode/syntaxes/ush.tmLanguage.json")),
        ("extension/language-configuration.json", include_bytes!("../../../editors/vscode/language-configuration.json")),
        ("extension/README.md", include_bytes!("../../../editors/vscode/README.md")),
        ("extension/LICENSE.txt", include_bytes!("../../../LICENSE")),
    ]))
}
