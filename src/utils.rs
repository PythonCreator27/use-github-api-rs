#[doc(hidden)]
#[macro_export]
macro_rules! check_for_errors {
    ($error_data:expr, $err:expr) => {{
        use crate::RuntimeError;
        #[cfg(feature = "auth")]
        if $error_data.message == "Bad credentials" {
            return Err(RuntimeError::bad_credentials().into());
        }
        if $error_data.message == "Not found" {
            return Err(RuntimeError::not_found().into());
        }
        return Err($err.into());
    }};
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    #[derive(Debug)]
    struct FakeError;

    impl std::fmt::Display for FakeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    impl Error for FakeError {}
    use serde_json::from_str;

    use crate::{error::runtime::RuntimeErrorKind, schema::GitHubError, RuntimeError};

    fn get_and_check_err(
        err: Result<(), Box<dyn Error>>,
        callback: fn(kind: &RuntimeErrorKind) -> (),
    ) {
        match err {
            Err(e) => match (*e).downcast_ref().unwrap() {
                RuntimeError { kind } => callback(kind),
            },
            _ => unreachable!(),
        };
    }

    #[test]
    #[cfg(feature = "auth")]
    fn returns_bad_creds() {
        let error_data = from_str::<GitHubError>(
            r#"{ "message": "Bad credentials", "documentation_url": "https://docs.github.com/rest" }"#,
        ).unwrap();
        let err = (move || -> Result<(), Box<dyn Error>> {
            check_for_errors!(error_data, FakeError {})
        })();
        get_and_check_err(err, |kind| {
            assert!(matches!(kind, RuntimeErrorKind::BadCredentials))
        });
    }

    #[test]
    fn returns_not_found() {
        let error_data = from_str::<GitHubError>(
            r#"{ "message": "Not found", "documentation_url": "https://docs.github.com/rest" }"#,
        )
        .unwrap();

        let err = (move || -> Result<(), Box<dyn Error>> {
            check_for_errors!(error_data, FakeError {})
        })();
        get_and_check_err(err, |kind| {
            assert!(matches!(kind, RuntimeErrorKind::NotFound))
        });
    }
}
