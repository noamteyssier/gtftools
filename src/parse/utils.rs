use anyhow::Result;

pub fn parse_to_usize(value: &[u8]) -> Result<usize> {
    let usize_value = std::str::from_utf8(value)?.parse()?;
    Ok(usize_value)
}
