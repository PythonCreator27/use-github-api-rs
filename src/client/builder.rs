use std::error::Error as StdError;

// This should work without this `cfg`, but the doctest fails without it (even though it is ignored).
#[cfg(feature = "auth")]
use crate::CreationError;
use crate::GithubClient;

#[non_exhaustive]
/// A `GithubClient` builder.
/// Allows building a `GithubClient` while still setting options like the base URL and auth token. Only useful when either the `auth` or `enterprise` feature enabled.
/// # Examples
/// ```rust,ignore
/// use use_github_api::{GithubClient, GithubClientBuilder};
/// // If `enterprise` is enabled
/// let client = GithubClientBuilder::new().auth("adS*lkjha(&W3").base_url("https://gh.enterprise.org/api/v3").build().unwrap();
/// // If `enterprise` is not enabled
/// let client = GithubClientBuilder::new().auth("ghp_kajshdkja").build().unwrap();
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "auth")))]
pub struct GithubClientBuilder<'a> {
    #[cfg(feature = "enterprise")]
    base_url: Option<&'a str>,
    #[cfg(feature = "auth")]
    auth_token: Option<&'a str>,
}

impl<'a> GithubClientBuilder<'a> {
    /// Creates a new `GithubClientBuilder`.
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "auth")]
            auth_token: None,
            #[cfg(feature = "enterprise")]
            base_url: None,
        }
    }

    /// Builds the builder and returns a client.
    /// # Errors
    /// If either the auth token or the base url is missing, this will error out.
    pub fn build(self) -> Result<GithubClient<'a>, Box<dyn StdError>> {
        #[cfg(all(feature = "auth", feature = "enterprise"))]
        return match self.auth_token {
            None => Err(CreationError::auth_token_not_provided().into()),
            Some(token) => match self.base_url {
                None => Err(CreationError::base_url_not_provided().into()),
                Some(base_url) => GithubClient::new(base_url, token),
            },
        };
        #[cfg(feature = "auth")]
        #[cfg(not(feature = "enterprise"))]
        return match self.auth_token {
            None => Err(CreationError::auth_token_not_provided().into()),
            Some(token) => GithubClient::new(token),
        };
    }

    #[cfg(any(feature = "auth", doc))]
    /// Sets the auth token.
    /// # Examples
    /// ```rust
    /// # use use_github_api::GithubClientBuilder;
    /// let builder = GithubClientBuilder::new();
    /// let builder = builder.auth("my auth token");
    /// // Build client and do stuff
    /// ```
    pub fn auth(mut self, auth_token: &'a str) -> Self {
        self.auth_token = Some(auth_token);
        self
    }

    #[cfg(any(feature = "enterprise", doc))]
    /// Sets the base url.
    /// # Examples
    /// ```rust
    /// # use use_github_api::GithubClientBuilder;
    /// let builder = GithubClientBuilder::new();
    /// let builder = builder.base_url("https://something.com/api/v3");
    /// // Build client and do stuff
    /// ```
    pub fn base_url(mut self, base_url: &'a str) -> Self {
        self.base_url = Some(base_url);
        self
    }
}

impl<'a> Default for GithubClientBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::FAKE_TOKEN;

    use super::*;
    #[test]
    fn creates_new() {
        let builder = GithubClientBuilder::new();
        assert_eq!(builder.auth_token, None);
        #[cfg(feature = "enterprise")]
        assert_eq!(builder.base_url, None);
    }

    #[test]
    fn sets_auth() {
        let builder = GithubClientBuilder::new();
        let token = "Some token";
        assert_eq!(builder.auth(token).auth_token, Some(token));
    }

    #[test]
    #[cfg(feature = "enterprise")]
    fn sets_base_url() {
        let builder = GithubClientBuilder::new();
        let base_url = "something.com";
        assert_eq!(builder.base_url(base_url).base_url, Some(base_url));
    }

    #[test]
    #[should_panic(expected = "CreationError { kind: AuthTokenNotProvided }")]
    fn err_on_no_token() {
        let builder = GithubClientBuilder::new();
        builder.build().unwrap();
    }

    #[test]
    #[cfg(feature = "enterprise")]
    #[should_panic(expected = "CreationError { kind: BaseUrlNotProvided }")]
    fn err_on_no_base_url() {
        let builder = GithubClientBuilder::new();
        builder.auth(FAKE_TOKEN).build().unwrap();
    }

    #[test]
    fn builds_client() {
        let builder = GithubClientBuilder::new();
        #[cfg(not(feature = "enterprise"))]
        let client = builder
            .auth(FAKE_TOKEN)
            .build()
            .expect("Should build client");
        #[cfg(feature = "enterprise")]
        let client = builder
            .auth(FAKE_TOKEN)
            .base_url("https://something.something.com/api/v3")
            .build()
            .expect("Should build client");
        assert_eq!(client.auth_token, FAKE_TOKEN);
        #[cfg(feature = "enterprise")]
        assert_eq!(client.base_url, "https://something.something.com/api/v3");
    }
}
