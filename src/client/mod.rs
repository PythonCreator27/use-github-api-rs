use crate::users::Users;
#[cfg(feature = "enterprise")]
use crate::CreationError;
use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client,
};
use std::error::Error as StdError;

#[cfg(any(feature = "auth", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "auth")))]
mod builder;

#[cfg(any(feature = "auth", doc))]
pub use builder::GithubClientBuilder;

pub(crate) mod macros {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! url {
        ($self:expr, $i:expr) => {
            format!("{}{}", $self.client.base_url, $i)
        };
        ($self:expr, $i:expr, $($arg:expr),*) => {
            format!("{}{}", $self.client.base_url, format!($i, $($arg),*))
        };
    }
}

#[derive(Debug)]
/// Holds the reqwest client, auth token, base url, headers, user agent, etc.
pub struct GithubClient<'a> {
    pub(crate) base_url: &'a str,
    pub(crate) reqwest_client: Client,
    #[cfg(feature = "auth")]
    auth_token: &'a str,
    pub(crate) default_headers: HeaderMap,
    user_agent: &'a str,
}

impl<'a> GithubClient<'a> {
    /// Creates a new `GithubClient` which can be used to send requests.
    /// # Signature
    /// The signatures of the function changes when the features are changed.
    /// - When no features are enabled, the signature is `fn () -> Result<GithubClient<'a>, Box<dyn StdError>>`
    /// - When the auth feature is enabled, the signature is `fn (auth_token: &'a str) -> Result<GithubClient<'a>, Box<dyn StdError>>`
    /// - When the enterprise feature is enabled, the signature is fn `fn (base_url: &'a str, auth_token: &'a str) -> Result<GithubClient<'a>, Box<dyn StdError>>`
    /// # Arguments
    /// ## Base URL
    /// The base url can be like this: `https://somehostfor.github.enterprise.org/api/v3`.
    /// Do make sure to add the `https://` and the `/api/v3`.
    /// ## Auth Token
    /// If using a PAT (personal access token), you can obtain one from https://github.com/settings/tokens.
    /// # Errors
    /// Will error if the protocol is not `http://` or `https://`, and will also error if the base URL does not include `/api/v3`.
    /// Will also error if the reqwest client fails to build.
    /// # Examples
    /// ```rust
    /// use use_github_api::GithubClient;
    /// # #[cfg(feature = "auth")]
    /// # #[cfg(not(feature = "enterprise"))]
    /// let client = GithubClient::new("ghp_akjsdh").unwrap(); // DO NOT ACTUALLY HARDCODE TOKENS IN YOUR APP!!!
    /// // do something with `client`
    /// ```
    pub fn new(
        #[cfg(feature = "enterprise")] base_url: &'a str,
        #[cfg(feature = "auth")] auth_token: &'a str,
    ) -> Result<GithubClient<'a>, Box<dyn StdError>> {
        #[cfg(feature = "enterprise")]
        if !(base_url.starts_with("https://") || base_url.starts_with("http://")) {
            return Err(CreationError::base_url_without_protocol().into());
        }
        #[cfg(feature = "enterprise")]
        if !base_url.ends_with("/api/v3") {
            return Err(CreationError::base_url_without_api_path().into());
        }
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/vnd.github.v3+json".parse().unwrap());
        #[cfg(feature = "auth")]
        if let Ok(token_header) = format!("token {}", auth_token).parse() {
            headers.insert("Authorization", token_header);
        }
        const UA: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
        let client = Client::builder()
            .default_headers(headers.clone())
            .user_agent(UA)
            .build()?;
        Ok(Self {
            #[cfg(feature = "enterprise")]
            base_url,
            #[cfg(not(feature = "enterprise"))]
            base_url: "https://api.github.com",
            reqwest_client: client,
            #[cfg(feature = "auth")]
            auth_token,
            default_headers: headers,
            user_agent: UA,
        })
    }

    #[cfg(feature = "auth")]
    /// Gives a `GithubClientBuilder`, same as using `GithubClientBuilder::new()`.
    pub fn builder() -> GithubClientBuilder<'a> {
        GithubClientBuilder::new()
    }

    pub fn users(&self) -> Users<'_> {
        Users::new(&self)
    }
}

#[cfg(not(feature = "auth"))]
impl<'a> Default for GithubClient<'a> {
    fn default() -> Self {
        Self::new().expect("Error while creating default client")
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "auth")]
    use crate::constants::FAKE_TOKEN;

    use super::*;
    #[test]
    fn new_creates_client_correctly() {
        #[cfg(feature = "auth")]
        use reqwest::header::HeaderValue;
        let client = GithubClient::new(
            #[cfg(feature = "enterprise")]
            "https://something.com/api/v3",
            #[cfg(feature = "auth")]
            FAKE_TOKEN,
        )
        .expect("Should build client");
        #[cfg(feature = "auth")]
        assert_eq!(client.auth_token, FAKE_TOKEN);
        #[cfg(feature = "enterprise")]
        assert_eq!(client.base_url, "https://something.com/api/v3");
        #[cfg(feature = "auth")]
        assert_eq!(
            client.default_headers.get("Authorization"),
            Some(
                &format!("token {}", FAKE_TOKEN)
                    .parse::<HeaderValue>()
                    .unwrap()
            )
        );
        assert_eq!(
            client.user_agent,
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    #[cfg(feature = "auth")]
    fn setting_auth_token_sets_header() {
        #[cfg(not(feature = "enterprise"))]
        let client = GithubClient::new("abc").expect("Should build client");
        #[cfg(feature = "enterprise")]
        let client =
            GithubClient::new("https://abc.abc/api/v3", "abc").expect("Should build client");
        assert_eq!(
            client.default_headers.get("Authorization"),
            Some(&"token abc".parse().unwrap())
        );
    }

    #[test]
    #[cfg(feature = "enterprise")]
    #[should_panic(expected = "CreationError { kind: BaseUrlWithoutProtocol }")]
    fn new_errors_on_no_protocol() {
        GithubClient::new("something", FAKE_TOKEN).expect("Should not work");
    }

    #[test]
    #[cfg(feature = "enterprise")]
    #[should_panic(expected = "CreationError { kind: BaseUrlWithoutApiPath }")]
    fn new_errors_on_no_api_path() {
        GithubClient::new("https://something.com", FAKE_TOKEN).unwrap();
    }

    #[test]
    #[cfg(feature = "enterprise")]
    fn new_for_valid_enterprise_works() {
        GithubClient::new("https://something.com/api/v3", FAKE_TOKEN).unwrap();
    }
}
