
use std::result;

enum Version {
    Version1, Version2
}

fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
    match header.get(0) {
        None => Err("Invalid header length."),
        Some(&1) => Ok(Version::Version1),
        Some(&2) => Ok(Version::Version2),
        Some(_) => Err("Invalid header.")
    }
}
