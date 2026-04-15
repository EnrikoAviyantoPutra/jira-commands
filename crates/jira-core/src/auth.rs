/// Auth helper — token storage is handled via config file (chmod 600 on Unix).
/// This module is kept for future credential helper / keyring opt-in support.
pub struct Auth;

impl Auth {
    /// Placeholder — token is stored directly in JiraConfig.
    /// Kept for API compatibility and future keyring opt-in.
    pub fn migrate_keyring_token(_email: &str) -> Option<String> {
        None
    }
}
