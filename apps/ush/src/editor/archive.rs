//! Deterministic ZIP STORE writer for the small, fixed embedded VSIX file set.
//! No compression dependency or external archiver is needed on user machines.
pub(super) fn zip(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut directory = Vec::new();
    for (name, data) in files {
        let offset = out.len() as u32;
        let size = data.len() as u32;
        let crc = crc32(data);
        u32s(&mut out, &[0x04034b50]);
        u16s(&mut out, &[20, 0x800, 0, 0, 33]);
        u32s(&mut out, &[crc, size, size]);
        u16s(&mut out, &[name.len() as u16, 0]);
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(data);
        u32s(&mut directory, &[0x02014b50]);
        u16s(&mut directory, &[20, 20, 0x800, 0, 0, 33]);
        u32s(&mut directory, &[crc, size, size]);
        u16s(&mut directory, &[name.len() as u16, 0, 0, 0, 0]);
        u32s(&mut directory, &[0, offset]);
        directory.extend_from_slice(name.as_bytes());
    }
    let offset = out.len() as u32;
    out.extend_from_slice(&directory);
    u32s(&mut out, &[0x06054b50]);
    u16s(&mut out, &[0, 0, files.len() as u16, files.len() as u16]);
    u32s(&mut out, &[directory.len() as u32, offset]);
    u16s(&mut out, &[0]);
    out
}
fn u16s(out: &mut Vec<u8>, values: &[u16]) {
    for v in values {
        out.extend_from_slice(&v.to_le_bytes());
    }
}
fn u32s(out: &mut Vec<u8>, values: &[u32]) {
    for v in values {
        out.extend_from_slice(&v.to_le_bytes());
    }
}
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for b in bytes {
        crc ^= *b as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb88320u32 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}
