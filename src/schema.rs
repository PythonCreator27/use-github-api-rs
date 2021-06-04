pub mod users {
    #[derive(Debug, serde::Deserialize)]
    pub struct Plan {
        name: String,
        space: usize,
        collaborators: usize,
        private_repos: usize,
    }

    pub mod list {
        use std::error::Error;

        #[derive(serde::Deserialize, serde::Serialize, Debug)]
        #[allow(dead_code)]
        pub struct User {
            pub login: String,
            pub id: usize,
            #[cfg(feature = "node_ids")]
            node_id: String,
            // url: String,
            // html_url: String,
            // followers_url: String,
            // following_url: String,
            // gists_url: String,
            // starred_url: String,
            // subscriptions_url: String,
            // organizations_url: String,
            // repos_url: String,
            // events_url: String,
            // received_events_url: String,
            pub r#type: String,
            pub site_admin: bool,
        }

        #[derive(serde::Serialize, Debug)]
        #[non_exhaustive]
        pub struct Params {
            pub since: Option<usize>,
            pub per_page: Option<usize>,
        }

        impl Params {
            pub fn new(
                since: Option<usize>,
                per_page: Option<usize>,
            ) -> Result<Self, Box<dyn Error>> {
                if let Some(per_page) = per_page {
                    if per_page > 100 {
                        return Err(
                            crate::users::error::UsersError::per_page_bigger_than_100().into()
                        );
                    }
                }

                Ok(Self { since, per_page })
            }
        }
    }

    pub mod single {
        use std::usize;

        use chrono::{DateTime, Utc};

        #[derive(serde::Deserialize, Debug)]
        pub struct User {
            pub login: String,
            pub id: usize,
            // avatar_url: String,
            pub gravatar_id: String,
            // url: String,
            // html_url: String,
            // followers_url: String,
            // following_url: String,
            // gists_url: String,
            // starred_url: String,
            // subscriptions_url: String,
            // organizations_url: String,
            // repos_url: String,
            // events_url: String,
            // received_events_url: String,
            pub r#type: String,
            pub site_admin: bool,
            pub name: Option<String>,
            pub company: Option<String>,
            pub blog: String,
            pub location: String,
            pub email: Option<String>,
            pub hireable: Option<bool>,
            pub bio: Option<String>,
            pub twitter_username: Option<String>,
            pub public_repos: usize,
            pub public_gists: usize,
            pub followers: usize,
            pub following: usize,
            pub created_at: DateTime<Utc>,
            pub updated_at: DateTime<Utc>,
            pub plan: Option<super::Plan>,
        }
    }

    pub mod contextual_info {
        #[derive(Debug, serde::Deserialize)]
        pub struct User {
            pub contexts: Vec<Context>,
        }

        #[derive(Debug, serde::Deserialize)]
        pub struct Context {
            pub message: String,
            pub octicon: String,
        }

        #[derive(Debug, serde::Serialize)]
        #[non_exhaustive]
        pub struct Params {
            subject_type: String,
            subject_id: usize,
        }

        impl Params {
            pub fn new(subject_type: String, subject_id: usize) -> Self {
                Self {
                    subject_type,
                    subject_id,
                }
            }
        }
    }

    pub mod current {
        use chrono::{DateTime, Utc};

        #[derive(serde::Deserialize, Debug)]
        pub struct User {
            pub login: String,
            pub id: usize,
            #[cfg(feature = "node_ids")]
            pub node_id: String,
            // avatar_url: String,
            // gravatar_id: String,
            // url: String,
            // html_url: String,
            // followers_url: String,
            // following_url: String,
            // gists_url: String,
            // starred_url: String,
            // subscriptions_url: String,
            // organizations_url: String,
            // repos_url: String,
            // received_events_url: String,
            pub r#type: String,
            pub site_admin: bool,
            pub name: Option<String>,
            pub company: Option<String>,
            pub blog: String,
            pub location: Option<String>,
            pub email: Option<String>,
            pub hireable: Option<bool>,
            pub bio: Option<String>,
            pub twitter_username: Option<String>,
            pub public_repos: usize,
            pub public_gists: usize,
            pub followers: usize,
            pub following: usize,
            pub created_at: DateTime<Utc>,
            pub updated_at: DateTime<Utc>,
            pub private_gists: Option<usize>,
            pub total_private_repos: Option<usize>,
            pub owned_private_repos: Option<usize>,
            pub disk_usage: Option<usize>,
            pub collaborators: Option<usize>,
            pub two_factor_authentication: Option<bool>,
            pub plan: Option<super::Plan>,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct GitHubError {
    pub message: String,
    pub documentation_url: String,
}
