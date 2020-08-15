pub enum ExitCode {
    Success,
    Failure,
}

impl Into<i32> for ExitCode {
    fn into(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::Failure => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let success: i32 = ExitCode::Success.into();
        let failure: i32 = ExitCode::Failure.into();

        assert_eq!(success, 0);
        assert_eq!(failure, 1);
    }
}
