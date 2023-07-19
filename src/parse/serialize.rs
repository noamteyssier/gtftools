use serde::Serializer;

pub fn parse_bytes<S>(x: &[u8], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let str_rep = std::str::from_utf8(x).expect("Could not parse bytes");
    s.serialize_str(str_rep)
}

pub fn parse_optional_bytes<S>(x: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        Some(x) => parse_bytes(x, s),
        None => s.serialize_none(),
    }
}
