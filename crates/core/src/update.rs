//! Deciding whether an update should be offered.
//! Pure descriptor parsing and validation logic — zero I/O, zero sockets, zero Win32.

use serde::Deserialize;

pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// Split `https://host/path` into its host and path. `None` for anything else.
pub fn split_https(url: &str) -> Option<(&str, &str)> {
    let rest = url.strip_prefix("https://")?;
    Some(match rest.split_once('/') {
        Some((host, path)) => (host, path),
        None => (rest, ""),
    })
}

/// Ceiling for the descriptor.
pub const DESCRIPTOR_LIMIT: u64 = 64 * 1024;

/// A release, as described by `latest.json`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Release {
    pub version: String,
    #[serde(default)]
    pub released: String,
    pub setup_url: String,
    pub setup_sha256: String,
    #[serde(default)]
    pub notes_url: String,
}

/// Why a release descriptor was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejected {
    /// The descriptor could not be parsed at all.
    Unreadable,
    /// A version string was not three numeric fields.
    BadVersion(String),
    /// `setup_sha256` was not 64 hexadecimal characters.
    BadChecksum,
    /// `setup_url` was not an HTTPS URL on the expected repository.
    BadUrl,
}

/// The outcome of asking whether to offer an update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Nothing to do; the running version is current or ahead.
    UpToDate,
    /// A newer, validated release is available.
    Available(Release),
    /// The descriptor was refused. Nothing is downloaded.
    Refused(Rejected),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Version(u64, u64, u64);

fn parse_version(raw: &str) -> Option<Version> {
    let trimmed = raw.trim().trim_start_matches('v');
    let mut parts = trimmed.split('.');
    let mut next = || -> Option<u64> {
        let field = parts.next()?;
        if field.is_empty() || !field.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        field.parse().ok()
    };
    let v = Version(next()?, next()?, next()?);
    if parts.next().is_some() {
        return None;
    }
    Some(v)
}

fn checksum_looks_like_sha256(raw: &str) -> bool {
    raw.len() == 64 && raw.bytes().all(|b| b.is_ascii_hexdigit())
}

fn required_download_prefix() -> Option<String> {
    let (_, repo_path) = split_https(REPOSITORY)?;
    Some(format!(
        "/{}/releases/download/",
        repo_path.trim_matches('/')
    ))
}

pub fn url_is_acceptable(raw: &str) -> bool {
    let Some((repo_host, _)) = split_https(REPOSITORY) else {
        return false;
    };
    let Some(prefix) = required_download_prefix() else {
        return false;
    };
    let Some((authority, path)) = split_https(raw) else {
        return false;
    };

    if authority != repo_host {
        return false;
    }

    let path = format!("/{path}");
    if path.contains("/../") || path.ends_with("/..") {
        return false;
    }
    path.starts_with(&prefix) && path.len() > prefix.len()
}

/// Validate a descriptor and decide against the running version.
pub fn decide(running: &str, descriptor: &str) -> Decision {
    let release: Release = match serde_json::from_str(descriptor) {
        Ok(r) => r,
        Err(_) => return Decision::Refused(Rejected::Unreadable),
    };

    let Some(running_v) = parse_version(running) else {
        return Decision::Refused(Rejected::BadVersion(running.to_owned()));
    };
    let Some(offered_v) = parse_version(&release.version) else {
        return Decision::Refused(Rejected::BadVersion(release.version.clone()));
    };

    if !checksum_looks_like_sha256(&release.setup_sha256) {
        return Decision::Refused(Rejected::BadChecksum);
    }
    if !url_is_acceptable(&release.setup_url) {
        return Decision::Refused(Rejected::BadUrl);
    }

    if offered_v > running_v {
        Decision::Available(release)
    } else {
        Decision::UpToDate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD_SHA: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn good_url() -> String {
        format!("{REPOSITORY}/releases/download/v0.2.0/WiraDesk-0.2.0-x64-setup.exe")
    }

    #[test]
    fn parses_valid_version() {
        assert_eq!(parse_version("0.2.3"), Some(Version(0, 2, 3)));
        assert_eq!(parse_version("v0.2.3"), Some(Version(0, 2, 3)));
        assert_eq!(parse_version("0.2"), None);
        assert_eq!(parse_version("0.2.3.4"), None);
    }

    #[test]
    fn decides_available_when_newer() {
        let json = format!(
            r#"{{"version": "0.3.0", "setup_url": "{}", "setup_sha256": "{}"}}"#,
            good_url(),
            GOOD_SHA
        );
        match decide("0.2.0", &json) {
            Decision::Available(rel) => assert_eq!(rel.version, "0.3.0"),
            other => panic!("expected Available, got {other:?}"),
        }
    }

    #[test]
    fn decides_up_to_date_when_same_or_older() {
        let json = format!(
            r#"{{"version": "0.2.0", "setup_url": "{}", "setup_sha256": "{}"}}"#,
            good_url(),
            GOOD_SHA
        );
        assert_eq!(decide("0.2.0", &json), Decision::UpToDate);
        assert_eq!(decide("0.3.0", &json), Decision::UpToDate);
    }
}
