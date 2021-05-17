#[cfg(not(test))]
use crate::url;
use crate::{check_for_errors, GithubClient};
use serde_json::from_str;
use std::error::Error;

#[non_exhaustive]
pub struct Users<'a> {
    #[cfg_attr(test, allow(dead_code))]
    client: &'a GithubClient<'a>,
}

use crate::schema::{
    users::{list, single},
    GitHubError,
};
impl<'a> Users<'a> {
    pub(crate) fn new(client: &'a GithubClient<'a>) -> Self {
        Users { client }
    }

    /// Fetches a list of users.
    pub async fn list(&self, cfg: Option<list::Params>) -> Result<Vec<list::User>, Box<dyn Error>> {
        #[cfg(test)]
        let text = crate::mock_response!(&self, "users", "list", cfg);
        #[cfg(not(test))]
        let text = {
            let result = self
                .client
                .reqwest_client
                .get(url!(self, "/users"))
                .query(&cfg)
                .send()
                .await?;
            result.text().await?
        };
        let json_result = from_str::<Vec<list::User>>(&text);
        match json_result {
            Ok(data) => Ok(data),
            Err(err) => {
                if err.is_data() {
                    let error_data = from_str::<GitHubError>(&text)?;
                    check_for_errors!(error_data, err);
                } else {
                    Err(err.into())
                }
            }
        }
    }

    /// Fetches a specific user.
    /// If authenticated, it will show a few more fields.
    /// If the current authenticated user is the same as the user being fetched, a few more fields will exist.
    /// # Errors
    /// Will error if the user does not exist.
    pub async fn user(&self, username: &str) -> Result<single::User, Box<dyn Error>> {
        #[cfg(test)]
        let text = crate::mock_response!(&self, "users", "user", username);
        #[cfg(not(test))]
        let text = {
            let result = self
                .client
                .reqwest_client
                .get(url!(self, "/users/{}", username))
                .send()
                .await?;
            result.text().await?
        };

        match from_str::<single::User>(&text) {
            Ok(data) => Ok(data),
            Err(err) => {
                if err.is_data() {
                    let error_data = from_str::<GitHubError>(&text)?;
                    check_for_errors!(error_data, err);
                } else {
                    Err(err.into())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "auth")]
    use crate::constants::{BAD_FAKE_TOKEN, FAKE_TOKEN};

    use super::*;

    #[tokio::test]
    #[cfg(not(feature = "enterprise"))]
    async fn list_works() {
        #[cfg(feature = "auth")]
        let client = GithubClient::new(FAKE_TOKEN).unwrap();

        #[cfg(not(feature = "auth"))]
        let client = GithubClient::new().unwrap();
        let users = Users::new(&client);
        let data = users.list(None).await.unwrap();
        assert_eq!(data[0].login, "mojombo");
        assert_eq!(data[0].id, 1);
    }

    #[tokio::test]
    #[cfg(feature = "auth")]
    #[should_panic(expected = "RuntimeError { kind: BadCredentials }")]
    async fn list_auth_returns_auth_err() {
        let client = GithubClient::new(
            #[cfg(feature = "enterprise")]
            "https://something.com/api/v3",
            BAD_FAKE_TOKEN,
        )
        .unwrap();

        let users = Users::new(&client);
        users.list(None).await.unwrap();
    }

    #[tokio::test]
    #[cfg(not(feature = "enterprise"))]
    async fn single_works() {
        #[cfg(feature = "auth")]
        let client = GithubClient::new(FAKE_TOKEN).unwrap();

        #[cfg(not(feature = "auth"))]
        let client = GithubClient::new().unwrap();
        let users = Users::new(&client);
        let data = users.user("mojombo").await.unwrap();
        assert_eq!(data.login, "mojombo");
    }
}
