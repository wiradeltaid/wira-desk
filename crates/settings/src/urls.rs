//! Canonical embedded URLs for Settings.
//!
//! All `wiradelta.id` endpoints terminate with a trailing slash (`/`) to avoid
//! unnecessary HTTP redirect hops. GitHub URLs are exempt from trailing slashes
//! where GitHub's routing standard does not require them (e.g. `/issues`), while
//! the repository root uses a trailing slash (`https://github.com/wiradeltaid/wira-desk/`).

/// Studio home page.
pub const WIRADELTA_HOME_URL: &str = "https://wiradelta.id/";

/// Product home page on the studio site.
pub const WIRA_DESK_PRODUCT_URL: &str = "https://wiradelta.id/wira-desk/";

/// Product privacy disclosure page.
pub const WIRA_DESK_PRIVACY_URL: &str = "https://wiradelta.id/wira-desk/privacy/";

/// Open source repository root on GitHub.
pub const GITHUB_REPO_URL: &str = "https://github.com/wiradeltaid/wira-desk/";

/// Public issue tracker on GitHub.
pub const GITHUB_ISSUES_URL: &str = "https://github.com/wiradeltaid/wira-desk/issues";

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    pub fn all_wiradelta_urls_end_with_slash() {
        let urls = [
            WIRADELTA_HOME_URL,
            WIRA_DESK_PRODUCT_URL,
            WIRA_DESK_PRIVACY_URL,
        ];
        for url in urls {
            assert!(
                url.starts_with("https://wiradelta.id/"),
                "{url} must start with https://wiradelta.id/"
            );
            assert!(
                url.ends_with('/'),
                "{url} must terminate with a trailing slash"
            );
        }
    }
}
