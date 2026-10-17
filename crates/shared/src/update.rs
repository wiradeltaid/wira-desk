//! Deciding whether an update should be offered and network endpoint wiring.
//!
//! Re-exports pure decision logic from `wiradesk_core::update` and retains
//! network endpoint constants, debug URL overrides, and live smoke tests.

pub use wiradesk_core::update::{decide, split_https, Decision, Rejected, Release, REPOSITORY};

/// Read only in debug builds, and only by `latest_json_url` below.
#[cfg(debug_assertions)]
const DEV_LATEST_JSON_URL_VAR: &str = "WIRADESK_DEV_LATEST_JSON_URL";

/// Canonical update descriptor endpoint for Wira Desk.
pub const UPDATE_DESCRIPTOR_URL: &str = "https://wiradelta.com/api/v1/update/wira-desk/";

/// Where the release descriptor lives. In release builds, this points to the canonical
/// self-hosted endpoint `https://wiradelta.com/api/v1/update/wira-desk/`.
pub fn latest_json_url() -> String {
    #[cfg(debug_assertions)]
    if let Ok(url) = std::env::var(DEV_LATEST_JSON_URL_VAR) {
        return url;
    }
    UPDATE_DESCRIPTOR_URL.to_owned()
}

/// Ceiling for the descriptor. It is a few hundred bytes; anything approaching this is
/// either not our file or not worth reading.
pub const DESCRIPTOR_LIMIT: u64 = 64 * 1024;

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD_SHA: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn good_url() -> String {
        format!("{REPOSITORY}/releases/download/v0.2.0/WiraDesk-0.2.0-x64-setup.exe")
    }

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
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcde",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdeg",
        ] {
            assert_eq!(
                decide("0.1.0", &descriptor("0.2.0", &good_url(), bad)),
                Decision::Refused(Rejected::BadChecksum),
                "{bad:?} should have been refused"
            );
        }
    }

    #[test]
    fn a_download_url_off_this_repository_is_refused() {
        let host = repo_host();
        let path = repo_path();
        let tail = "releases/download/v0.2.0/x.exe";

        let bad_urls = vec![
            format!("http://{host}/{path}/{tail}"),
            format!("https://evil-{host}/{path}/{tail}"),
            format!("https://{host}.evil.tld/{path}/{tail}"),
            format!("https://{host}@evil.tld/{path}/{tail}"),
            format!("https://{host}:8443/{path}/{tail}"),
            format!("https://{host}/someone/else/{tail}"),
            format!("https://{host}/{path}/raw/main/x.exe"),
            format!("https://{host}/{path}/releases/download/../../../x.exe"),
            format!("https://{host}/{path}/releases/download/"),
            format!("{host}/{path}/{tail}"),
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
    fn descriptor_url_is_wiradelta_endpoint() {
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
