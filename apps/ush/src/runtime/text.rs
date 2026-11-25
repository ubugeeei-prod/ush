use super::{arity, index, wire};
use anyhow::{Result, bail, ensure};

pub(super) fn run(name: &str, args: &[String]) -> Result<String> {
    arity(
        args,
        match name {
            "slice" => 3,
            "find" => 2,
            _ => 1,
        },
    )?;
    let text = &args[0];
    Ok(match name {
        "length" => text.chars().count().to_string(),
        "byte_length" => text.len().to_string(),
        "chars" => wire::cells(text.chars().map(|c| c.to_string())),
        "reverse" => text.chars().rev().collect(),
        "to_lower" => text.to_lowercase(),
        "to_upper" => text.to_uppercase(),
        "find" => text
            .find(&args[1])
            .map_or_else(|| "-1".into(), |i| text[..i].chars().count().to_string()),
        "slice" => {
            let start = index(&args[1])?;
            let end = index(&args[2])?;
            ensure!(
                start <= end && end <= text.chars().count(),
                "slice range must satisfy 0 <= start <= end <= character length"
            );
            text.chars().skip(start).take(end - start).collect()
        }
        _ => bail!("unknown string operation: {name}"),
    })
}
