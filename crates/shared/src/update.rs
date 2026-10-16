//! Deciding whether an update should be offered.
//!
//! In `shared` because both binaries need it: the daemon asks the question on its daily
//! check, and `settings` asks it again when a person presses the button. The rules that
//! refuse a tampered descriptor are the last thing to keep two copies of.
//!
//! This module is deliberately the whole decision and none of the plumbing: it parses a
//! release descriptor, compares it against the running version, and validates the thing it
//! is being told to download. No sockets, no files, no Win32 — so every rule below is
//! reachable from a unit test, which is the point. The failures this layer prevents are
//! silent ones, and a silent failure that only a live network can produce is a failure
//! nobody tests.
//!
//! The layers on top of it, added separately: fetching `latest.json` over HTTPS, hashing a
//! download, verifying an Authenticode signature once a certificate exists, and launching
//! the installer detached.
//!
//! # What `latest.json` is
//! Published by the release workflow as a release asset under a name that never changes, so
//! `releases/latest/download/latest.json` is a permanently stable URL. One file is read by
//! both this updater and the download page on the website, so the two cannot disagree about
//! what the current version is.

pub use wiradesk_core::update::*;

/// The repository this build belongs to, taken from `Cargo.toml`.
///
/// Read with `env!` rather than written out here, and not only to avoid a duplicate that
/// could drift. `scripts/verify-public-export.ps1` permits the maintainer's handle in exactly
/// two places, one of them a crate manifest's `repository` line — so deriving the value keeps
/// the fact in its declared home and keeps this file free of a second copy. The gate caught
/// the first version of this module for exactly that.
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

// Split `https://host/path` into its host and path is re-exported from `wiradesk_core::update::split_https`.

/// Read only in debug builds, and only by `latest_json_url` below.
#[cfg(debug_assertions)]
const DEV_LATEST_JSON_URL_VAR: &str = "WIRADESK_DEV_LATEST_JSON_URL";

/// Canonical update descriptor endpoint for Wira Desk.
pub const UPDATE_DESCRIPTOR_URL: &str = "https://wiradelta.com/api/v1/update/wira-desk/";

/// Where the release descriptor lives. In release builds, this points to the canonical
/// self-hosted endpoint `https://wiradelta.com/api/v1/update/wira-desk/`.
///
/// **Debug builds only:** `WIRADESK_DEV_LATEST_JSON_URL`, when set, is returned instead of the
/// real address. This is the seam that lets check → download → verify → launch be exercised
/// against a stand-in descriptor before any real tag exists — before which the real URL below
/// has nothing to answer with. `#[cfg(debug_assertions)]` compiles the branch, and the
/// `std::env::var` call inside it, out of a release build entirely: there is no environment
/// variable anywhere in a shipped copy of this product that can redirect where it fetches an
/// installer from. Same shape as the debug-only seams already in `daemon::tray`.
pub fn latest_json_url() -> String {
    #[cfg(debug_assertions)]
    if let Ok(url) = std::env::var(DEV_LATEST_JSON_URL_VAR) {
        return url;
    }
    UPDATE_DESCRIPTOR_URL.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD_SHA: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    /// A URL of the shape the release workflow actually emits, built from the same
    /// `Cargo.toml` value the guard reads. Derived rather than written out for the reason
    /// given at `REPOSITORY`, and with the useful side effect that a repository move keeps
    /// these tests meaningful instead of quietly testing a stale address.
    fn good_url() -> String {
        format!("{REPOSITORY}/releases/download/v0.2.0/WiraDesk-0.2.0-x64-setup.exe")
    }

    /// The host part of the repository URL, for building deliberately wrong variants.
    fn repo_host() -> String {
        split_https(REPOSITORY).unwrap().0.to_owned()
    }

    fn repo_path() -> String {
        split_https(REPOSITORY)
            .unwrap()
            .1
            .trim_matches('/')
            .to_owned()
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
    fn a_newer_version_is_offered() {
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

    /// The replay case, and the reason the comparison is `>` and not `!=`. A descriptor
    /// naming an older release would offer an installer that is genuinely this product's,
    /// correctly hashed and one day correctly signed — so ordering is the only thing that
    /// refuses it.
    #[test]
    fn an_older_version_is_never_offered() {
        assert_eq!(decide("0.2.0", &good("0.1.9")), Decision::UpToDate);
        assert_eq!(decide("1.0.0", &good("0.9.9")), Decision::UpToDate);
    }

    #[test]
    fn version_fields_are_compared_numerically_not_as_text() {
        // "0.10.0" sorts before "0.9.0" as a string and after it as a version.
        assert!(matches!(
            decide("0.9.0", &good("0.10.0")),
            Decision::Available(_)
        ));
        assert_eq!(decide("0.10.0", &good("0.9.0")), Decision::UpToDate);
    }

    #[test]
    fn a_leading_v_is_tolerated_on_either_side() {
        assert!(matches!(
            decide("v0.1.0", &good("v0.2.0")),
            Decision::Available(_)
        ));
    }

    #[test]
    fn unparseable_json_is_refused() {
        assert_eq!(
            decide("0.1.0", "not json at all"),
            Decision::Refused(Rejected::Unreadable)
        );
        // A well-formed document missing a required field is equally unusable.
        assert_eq!(
            decide("0.1.0", r#"{"version":"0.2.0"}"#),
            Decision::Refused(Rejected::Unreadable)
        );
    }

    #[test]
    fn a_version_that_is_not_three_numbers_is_refused() {
        for bad in ["", "0.2", "0.2.0.1", "0.2.x", "latest", "0..0", "0.2.-1"] {
            assert!(
                matches!(
                    decide("0.1.0", &good(bad)),
                    Decision::Refused(Rejected::BadVersion(_))
                ),
                "{bad:?} should have been refused"
            );
        }
    }

    #[test]
    fn a_checksum_that_is_not_sha256_is_refused() {
        for bad in [
            "",
            "abc",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcde", // 63
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0", // 65
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdeg", // non-hex
        ] {
            assert_eq!(
                decide("0.1.0", &descriptor("0.2.0", &good_url(), bad)),
                Decision::Refused(Rejected::BadChecksum),
                "{bad:?} should have been refused"
            );
        }
    }

    /// Each of these is a way a tampered descriptor could try to send the download
    /// somewhere else while still looking plausible in a log line.
    #[test]
    fn a_download_url_off_this_repository_is_refused() {
        let host = repo_host();
        let path = repo_path();
        let tail = "releases/download/v0.2.0/x.exe";

        let bad_urls = vec![
            // Not HTTPS.
            format!("http://{host}/{path}/{tail}"),
            // A different host that ends with the right letters.
            format!("https://evil-{host}/{path}/{tail}"),
            // A different host that begins with them.
            format!("https://{host}.evil.tld/{path}/{tail}"),
            // The real host hidden behind userinfo.
            format!("https://{host}@evil.tld/{path}/{tail}"),
            // The real host with a port, which the workflow never emits.
            format!("https://{host}:8443/{path}/{tail}"),
            // Right host, someone else's project.
            format!("https://{host}/someone/else/{tail}"),
            // Right host and project, but not a release download.
            format!("https://{host}/{path}/raw/main/x.exe"),
            // Dot segments.
            format!("https://{host}/{path}/releases/download/../../../x.exe"),
            // The prefix and nothing after it.
            format!("https://{host}/{path}/releases/download/"),
            // No scheme at all.
            format!("{host}/{path}/{tail}"),
            // Host only.
            format!("https://{host}"),
        ];

        for url in bad_urls {
            assert_eq!(
                decide("0.1.0", &descriptor("0.2.0", &url, GOOD_SHA)),
                Decision::Refused(Rejected::BadUrl),
                "{url:?} should have been refused"
            );
        }
    }

    #[test]
    fn the_real_workflow_url_shape_is_accepted() {
        assert!(
            url_is_acceptable(&good_url()),
            "the guard must accept {:?}, which is what the release workflow writes",
            good_url()
        );
    }

    /// Validation runs before the comparison, so a descriptor that is both tampered with
    /// and older is reported as tampered with. The reverse order would hide an attack
    /// behind a reassuring "you are up to date", and the log would say nothing happened.
    #[test]
    fn validation_precedes_the_version_comparison() {
        assert_eq!(
            decide(
                "0.9.0",
                &descriptor("0.1.0", "http://evil.tld/x.exe", GOOD_SHA)
            ),
            Decision::Refused(Rejected::BadUrl),
            "an older descriptor with a foreign URL must report the URL, not up-to-date"
        );
        assert_eq!(
            decide("0.9.0", &descriptor("0.1.0", &good_url(), "nonsense")),
            Decision::Refused(Rejected::BadChecksum)
        );
    }

    #[test]
    fn descriptor_url_is_wiradelta_endpoint() {
        // Isolate test from ambient environment variables
        let prev = std::env::var(DEV_LATEST_JSON_URL_VAR).ok();
        std::env::remove_var(DEV_LATEST_JSON_URL_VAR);

        assert_eq!(
            latest_json_url(),
            "https://wiradelta.com/api/v1/update/wira-desk/"
        );
        assert_eq!(
            UPDATE_DESCRIPTOR_URL,
            "https://wiradelta.com/api/v1/update/wira-desk/"
        );

        if let Some(val) = prev {
            std::env::set_var(DEV_LATEST_JSON_URL_VAR, val);
        }
    }

    /// Live smoke test for the debug-only seam on `latest_json_url`. Ignored by default,
    /// because it touches the network and a real, clearly-labelled test descriptor rather than
    /// anything synthetic -- everything above this line needs neither. Meaningful only with
    /// `WIRADESK_DEV_LATEST_JSON_URL` pointed at one:
    ///
    /// `WIRADESK_DEV_LATEST_JSON_URL=<url> cargo test -p shared -- --ignored the_debug_override_reaches_a_real_descriptor`
    #[test]
    #[ignore]
    fn the_debug_override_reaches_a_real_descriptor() {
        let url = std::env::var("WIRADESK_DEV_LATEST_JSON_URL")
            .expect("set WIRADESK_DEV_LATEST_JSON_URL to a real stand-in latest.json first");
        assert_eq!(
            latest_json_url(),
            url,
            "the override must be what latest_json_url() returns"
        );

        let body = crate::https::get_text(&url, DESCRIPTOR_LIMIT).expect("fetch failed");
        match decide("0.0.1", &body) {
            Decision::Available(r) => println!("offered {} at {}", r.version, r.setup_url),
            other => panic!("expected Available against a stand-in newer version, got {other:?}"),
        }
    }

    /// Live test confirming that descriptor fetches refuse redirects (e.g. 302 from GitHub latest download).
    ///
    /// `cargo test -p shared -- --ignored ignored_live_debug_override_fails_on_redirect`
    #[test]
    #[ignore]
    fn ignored_live_debug_override_fails_on_redirect() {
        let url = std::env::var("WIRADESK_DEV_LATEST_JSON_URL")
            .unwrap_or_else(|_| format!("{REPOSITORY}/releases/latest/download/latest.json"));
        let result = crate::https::get_text(&url, DESCRIPTOR_LIMIT);
        match result {
            Err(crate::https::HttpError::Status(code)) if (300..400).contains(&code) => {
                println!("Confirmed redirect refused with status {code}");
            }
            other => panic!("expected 3xx refusal without following redirect, got {other:?}"),
        }
    }
}
