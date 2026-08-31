#[cfg(test)]
mod tests;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::helpers::ValueStream;

use super::{
    common::normalize_path,
    ls_support::{HiddenMode, LsSummary, describe_ls_entry, render_ls_row, render_ls_section},
};

pub fn render_ls(cwd: &Path, args: &[String]) -> Result<Option<ValueStream>> {
    let Some((hidden_mode, mut targets)) = parse_ls_args(args) else {
        return Ok(None);
    };
    if targets.is_empty() {
        targets.push(".".to_string());
    }

    let mut sections = Vec::new();
    for target in targets {
        let path = normalize_path(cwd, &target);
        let mut entries = ls_entries(&path, hidden_mode)
            .with_context(|| format!("failed to read {}", path.display()))?;
        entries.sort_by(|left, right| left.0.cmp(&right.0));

        let mut summary = LsSummary::default();
        let mut body = String::new();
        for (file_name, entry_path) in entries {
            let row = describe_ls_entry(&file_name, &entry_path, hidden_mode)
                .with_context(|| format!("failed to read {}", entry_path.display()))?;
            summary.observe(row.kind);
            render_ls_row(&mut body, &row);
        }
        sections.push(render_ls_section(&target, &summary, &body));
    }

    Ok(Some(ValueStream::Text(sections.join("\n"))))
}

fn parse_ls_args(args: &[String]) -> Option<(HiddenMode, Vec<String>)> {
    let mut hidden_mode = HiddenMode::Default;
    let mut targets = Vec::new();
    let mut force_paths = false;
    for arg in args {
        if force_paths {
            targets.push(arg.clone());
            continue;
        }
        match arg.as_str() {
            "--" => force_paths = true,
            "--all" => hidden_mode = hidden_mode.include(HiddenMode::All),
            "--almost-all" => hidden_mode = hidden_mode.include(HiddenMode::AlmostAll),
            "--long" | "--human-readable" | "--classify" | "--file-type" | "--color" => {}
            _ if arg.starts_with("--color=") => {}
            _ if arg.starts_with("--indicator-style=") => match arg.split_once('=')?.1 {
                "classify" | "file-type" | "slash" => {}
                _ => return None,
            },
            _ if arg.starts_with('-') && arg.len() > 1 => {
                parse_ls_short_flags(arg, &mut hidden_mode)?
            }
            _ => targets.push(arg.clone()),
        }
    }
    Some((hidden_mode, targets))
}

fn parse_ls_short_flags(arg: &str, hidden_mode: &mut HiddenMode) -> Option<()> {
    for flag in arg[1..].chars() {
        match flag {
            'a' => *hidden_mode = hidden_mode.include(HiddenMode::All),
            'A' => *hidden_mode = hidden_mode.include(HiddenMode::AlmostAll),
            '1' | 'C' | 'F' | 'G' | 'h' | 'l' | 'm' | 'p' | 'x' => {}
            _ => return None,
        }
    }
    Some(())
}

fn ls_entries(path: &Path, hidden_mode: HiddenMode) -> Result<Vec<(String, PathBuf)>> {
    if path.is_dir() {
        let mut entries = fs::read_dir(path)?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter_map(|entry| {
                let file_name = entry.file_name().to_string_lossy().to_string();
                (!file_name.starts_with('.') || hidden_mode.shows_hidden())
                    .then_some((file_name, entry.path()))
            })
            .collect::<Vec<_>>();
        if hidden_mode.shows_dot_entries() {
            entries.push((".".to_string(), path.to_path_buf()));
            entries.push((
                "..".to_string(),
                path.parent().unwrap_or(path).to_path_buf(),
            ));
        }
        Ok(entries)
    } else {
        let file_name = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        Ok(vec![(file_name, path.to_path_buf())])
    }
}
