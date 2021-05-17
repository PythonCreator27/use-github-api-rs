#[cfg(not(test))]
use crate::url;
use crate::{check_for_errors, schema::users::contextual_info, GithubClient};
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

    #[cfg(any(feature = "auth", doc))]
    #[cfg_attr(docsrs, doc(cfg(feature = "auth")))]
    /// Fetches contextual info (like the hovercard you see on github). Requires auth.
    /// It can either work with the username alone (and then it will fetch profile data), or it can work with a subject type (like "repository") and id (the repository's id on the API).
    /// # Errors
    /// Will error if the user doesn't exist.
    pub async fn contextual_info(
        &self,
        username: &str,
        cfg: Option<contextual_info::Params>,
    ) -> Result<contextual_info::User, Box<dyn Error>> {
        #[cfg(test)]
        let text = crate::mock_response!(&self, "users", "contextual_info", (username, cfg));
        #[cfg(not(test))]
        let text = {
            let result = self
                .client
                .reqwest_client
                .get(url!(self, "/users/{}/hovercard", username))
                .query(&cfg)
                .send()
                .await?;
            result.text().await?
        };

        match from_str::<contextual_info::User>(&text) {
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
    use crate::constants::FAKE_TOKEN;

    use super::*;

    #[tokio::test]
    async fn list_works() {
        let client = GithubClient::new(
            #[cfg(feature = "enterprise")]
            "https://something.com/api/v3",
            #[cfg(feature = "auth")]
            FAKE_TOKEN,
        )
        .unwrap();

        let users = Users::new(&client);
        let data = users.list(None).await.unwrap();
        assert_eq!(data[0].login, "mojombo");
        assert_eq!(data[0].id, 1);
    }

    #[tokio::test]
    async fn single_works() {
        let client = GithubClient::new(
            #[cfg(feature = "enterprise")]
            "https://something.com/api/v3",
            #[cfg(feature = "auth")]
            FAKE_TOKEN,
        )
        .unwrap();
        let users = Users::new(&client);
        let data = users.user("mojombo").await.unwrap();
        assert_eq!(data.login, "mojombo");
    }

    #[tokio::test]
    #[cfg(feature = "auth")]
    async fn context_info_works() {
        #[cfg(feature = "auth")]
        let client = GithubClient::new(
            #[cfg(feature = "enterprise")]
            "https://something.com/api/v3",
            FAKE_TOKEN,
        )
        .unwrap();
        let users = Users::new(&client);
        let data = users.contextual_info("mojombo", None).await.unwrap();
        assert_eq!(data.contexts[0].message, "Member of @toml-lang");
    }
}
