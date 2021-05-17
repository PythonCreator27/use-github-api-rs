#[cfg(any(feature = "auth", feature = "enterprise"))]
pub mod creation {
    use std::{error::Error as StdError, fmt};

    #[derive(Debug)]
    pub(crate) enum CreationErrorKind {
        #[cfg(feature = "enterprise")]
        BaseUrlWithoutProtocol,
        #[cfg(feature = "enterprise")]
        BaseUrlWithoutApiPath,
        #[cfg(feature = "auth")]
        AuthTokenNotProvided,
        #[cfg(feature = "enterprise")]
        BaseUrlNotProvided,
    }

    #[derive(Debug)]
    pub struct CreationError {
        pub(crate) kind: CreationErrorKind,
    }

    impl CreationError {
        fn new(kind: CreationErrorKind) -> Self {
            Self { kind }
        }

        #[cfg(feature = "enterprise")]
        pub(crate) fn base_url_without_protocol() -> Self {
            Self::new(CreationErrorKind::BaseUrlWithoutProtocol)
        }

        #[cfg(feature = "enterprise")]
        pub(crate) fn base_url_without_api_path() -> Self {
            Self::new(CreationErrorKind::BaseUrlWithoutApiPath)
        }

        #[cfg(feature = "auth")]
        pub(crate) fn auth_token_not_provided() -> Self {
            Self::new(CreationErrorKind::AuthTokenNotProvided)
        }

        #[cfg(feature = "enterprise")]
        pub(crate) fn base_url_not_provided() -> Self {
            Self::new(CreationErrorKind::BaseUrlNotProvided)
        }
    }

    impl StdError for CreationError {}

    impl fmt::Display for CreationError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.kind {
                #[cfg(feature = "enterprise")]
                CreationErrorKind::BaseUrlWithoutProtocol => {
                    write!(f, "Base URL is without the protocol.")
                }

                #[cfg(feature = "enterprise")]
                CreationErrorKind::BaseUrlWithoutApiPath => {
                    write!(f, "Base URL is without the `/api/v3` path at the end.")
                }
                #[cfg(feature = "auth")]
                CreationErrorKind::AuthTokenNotProvided => {
                    write!(f, "Auth token not provided")
                }
                #[cfg(feature = "enterprise")]
                CreationErrorKind::BaseUrlNotProvided => {
                    write!(f, "Base URL is not provided.")
                }
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use crate::CreationError;

        fn assert_sync<T: Sync>() {}
        fn assert_send<T: Send>() {}

        #[test]
        fn test_send_and_sync() {
            assert_sync::<CreationError>();
            assert_send::<CreationError>();
        }
    }
}

pub mod runtime {
    use std::{error::Error as StdError, fmt};

    #[derive(Debug)]
    pub(crate) enum RuntimeErrorKind {
        #[cfg(feature = "auth")]
        BadCredentials,
        NotFound,
    }

    #[derive(Debug)]
    pub struct RuntimeError {
        pub(crate) kind: RuntimeErrorKind,
    }

    impl RuntimeError {
        fn new(kind: RuntimeErrorKind) -> Self {
            Self { kind }
        }

        #[cfg(feature = "auth")]
        pub(crate) fn bad_credentials() -> Self {
            Self::new(RuntimeErrorKind::BadCredentials)
        }

        pub(crate) fn not_found() -> Self {
            Self::new(RuntimeErrorKind::NotFound)
        }
    }

    impl StdError for RuntimeError {}

    impl fmt::Display for RuntimeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.kind {
                #[cfg(feature = "auth")]
                RuntimeErrorKind::BadCredentials => {
                    write!(f, "Bad credentials")
                }
                RuntimeErrorKind::NotFound => {
                    write!(f, "Either the resource does not exist, or it is protected")
                }
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use crate::RuntimeError;

        fn assert_sync<T: Sync>() {}
        fn assert_send<T: Send>() {}

        #[test]
        fn test_send_and_sync() {
            assert_sync::<RuntimeError>();
            assert_send::<RuntimeError>();
        }
    }
}
