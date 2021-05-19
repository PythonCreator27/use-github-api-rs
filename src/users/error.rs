use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub(crate) enum UsersErrorKind {
    PerPageBiggerThan100,
}

#[derive(Debug)]
pub struct UsersError {
    pub(crate) kind: UsersErrorKind,
}

impl UsersError {
    fn new(kind: UsersErrorKind) -> Self {
        Self { kind }
    }

    pub(crate) fn per_page_bigger_than_100() -> Self {
        Self::new(UsersErrorKind::PerPageBiggerThan100)
    }
}

impl StdError for UsersError {}

impl fmt::Display for UsersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            UsersErrorKind::PerPageBiggerThan100 => {
                write!(
                    f,
                    "per_page is bigger than 100. It has to be less than or equivalent to 100."
                )
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::UsersError;

    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_and_sync() {
        assert_sync::<UsersError>();
        assert_send::<UsersError>();
    }
}
