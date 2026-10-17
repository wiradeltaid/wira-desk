//! Deciding whether an update should be offered — pure sans-IO logic.
//!
//! Parses a release descriptor, compares it against the running version, and
//! validates the download instruction. No sockets, no files, no Win32.

use serde::Deserialize;

/// The repository this build belongs to, taken from `Cargo.toml`.
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// Split `https://host/path` into its host and path. `None` for anything else.
pub fn split_https(url: &str) -> Option<(&str, &str)> {
    let rest = url.strip_prefix("https://")?;
    Some(match rest.split_once('/') {
        Some((host, path)) => (host, path),
        None => (rest, ""),
    })
}

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

/// Three numeric fields, compared field by field.
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

/// The one prefix a download URL may begin with: this repository's release downloads.
fn required_download_prefix() -> Option<String> {
    let (_, repo_path) = split_https(REPOSITORY)?;
    Some(format!(
        "/{}/releases/download/",
        repo_path.trim_matches('/')
    ))
}

/// `true` when the URL is HTTPS and points into this project's own release downloads.
fn url_is_acceptable(raw: &str) -> bool {
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
pub mod tests {
    use super::*;

    const GOOD_SHA: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn good_url() -> String {
        format!("{REPOSITORY}/releases/download/v0.2.0/WiraDesk-0.2.0-x64-setup.exe")
    }

    fn descriptor(version: &str, url: &str, sha: &str) -> String {
        format!(
            r#"{{"version":"{version}","released":"2026-09-01","setup_url":"{url}",
               "setup_sha256":"{sha}","notes_url":"https://example.invalid/notes"}}"#
        )
    }

    fn good(version: &str) -> String {
        descriptor(version, &good_url(), GOOD_SHA)
    }

    #[test]
    pub fn test_descriptor_decision() {
        match decide("0.1.0", &good("0.2.0")) {
            Decision::Available(r) => {
                assert_eq!(r.version, "0.2.0");
                assert_eq!(r.setup_sha256, GOOD_SHA);
            }
            other => panic!("expected Available, got {other:?}"),
        }
    }

    #[test]
    fn the_same_version_is_up_to_date() {
        assert_eq!(decide("0.1.0", &good("0.1.0")), Decision::UpToDate);
    }

    #[test]
    fn an_older_version_is_never_offered() {
        assert_eq!(decide("0.2.0", &good("0.1.9")), Decision::UpToDate);
        assert_eq!(decide("1.0.0", &good("0.9.9")), Decision::UpToDate);
    }

    #[test]
    fn version_fields_are_compared_numerically_not_as_text() {
        assert!(matches!(
            decide("0.9.0", &good("0.10.0")),
            Decision::Available(_)
        ));
        assert_eq!(decide("0.10.0", &good("0.9.0")), Decision::UpToDate);
    }

    #[test]
    fn unparseable_json_is_refused() {
        assert_eq!(
            decide("0.1.0", "not json at all"),
            Decision::Refused(Rejected::Unreadable)
        );
        assert_eq!(
            decide("0.1.0", r#"{"version":"0.2.0"}"#),
            Decision::Refused(Rejected::Unreadable)
        );
    }

    #[test]
    fn bad_version_is_refused() {
        assert!(matches!(
            decide("0.1.0", &good("0.2.x")),
            Decision::Refused(Rejected::BadVersion(_))
        ));
    }

    #[test]
    fn bad_checksum_is_refused() {
        assert_eq!(
            decide("0.1.0", &descriptor("0.2.0", &good_url(), "bad")),
            Decision::Refused(Rejected::BadChecksum)
        );
    }
}
