use super::{arity, wire};
use anyhow::{Context, Result, bail, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};

pub(super) fn run(name: &str, args: &[String]) -> Result<String> {
    arity(args, 1)?;
    Ok(match name {
        "from_hex" => wire::bytes(&wire::unhex(&args[0])?),
        "from_text" => wire::bytes(args[0].as_bytes()),
        "decode_base64" => wire::bytes(&STANDARD.decode(&args[0]).context("invalid base64")?),
        "to_hex" => wire::hex(&wire::read_bytes(&args[0])?),
        "encode_base64" => STANDARD.encode(wire::read_bytes(&args[0])?),
        "to_text" => {
            let bytes = wire::read_bytes(&args[0])?;
            ensure!(
                !bytes.contains(&0),
                "String cannot contain NUL; keep this value as List<Int>"
            );
            String::from_utf8(bytes).context("bytes are not valid UTF-8")?
        }
        _ => bail!("unknown bytes operation: {name}"),
    })
}
