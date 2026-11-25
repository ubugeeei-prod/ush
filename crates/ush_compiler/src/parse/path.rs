use anyhow::{Result, bail};

use crate::types::AstString as String;

use super::super::util::{is_identifier, is_path, path_tail};

pub(super) fn looks_like_call_target(source: &str) -> bool {
    let trimmed = source.trim();
    is_identifier(trimmed)
        || is_path(trimmed)
        || trimmed.strip_prefix("$bin::").is_some_and(is_identifier)
}

pub(super) fn parse_call_target(source: &str) -> Result<String> {
    let trimmed = source.trim();
    if looks_like_call_target(trimmed) {
        Ok(trimmed.into())
    } else {
        bail!("invalid call target: {trimmed}")
    }
}

pub(super) fn parse_import_path(source: &str) -> Result<String> {
    let trimmed = source.trim();
    if is_path(trimmed) || is_identifier(trimmed) {
        Ok(trimmed.into())
    } else {
        bail!("invalid import path: {trimmed}")
    }
}

pub(super) fn parse_identifier(source: &str) -> Result<String> {
    let trimmed = source.trim();
    if is_identifier(trimmed) {
        Ok(trimmed.into())
    } else {
        bail!("invalid identifier: {trimmed}")
    }
}

pub(super) fn default_import_alias(path: &str) -> Result<String> {
    parse_identifier(path_tail(path).unwrap_or(path))
}
