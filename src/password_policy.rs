use thiserror::Error;

pub const MIN_PASSWORD_LEN: usize = 8;
pub const MAX_PASSWORD_LEN: usize = 128;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PasswordError {
    #[error("Password length must be between {min} and {max} characters")]
    InvalidLength { min: usize, max: usize },

    #[error("Password must contain at least one uppercase letter")]
    MissingUppercase,

    #[error("Password must contain at least one lowercase letter")]
    MissingLowercase,

    #[error("Password must contain at least one digit")]
    MissingDigit,

    #[error("Password is too weak")]
    TooWeak,
}

pub fn validate_password(password: &str) -> Result<(), PasswordError> {
    let len = password.len();
    if !(MIN_PASSWORD_LEN..=MAX_PASSWORD_LEN).contains(&len) {
        return Err(PasswordError::InvalidLength {
            min: MIN_PASSWORD_LEN,
            max: MAX_PASSWORD_LEN,
        });
    }

    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if !has_upper {
        return Err(PasswordError::MissingUppercase);
    }
    if !has_lower {
        return Err(PasswordError::MissingLowercase);
    }
    if !has_digit {
        return Err(PasswordError::MissingDigit);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        assert!(validate_password("Password123").is_ok());
        assert!(validate_password("MySecureP@ss1").is_ok());
    }

    #[test]
    fn test_password_too_short() {
        let result = validate_password("Pass1");
        assert!(matches!(result, Err(PasswordError::InvalidLength { .. })));
    }

    #[test]
    fn test_password_too_long() {
        let long_password = "a".repeat(MAX_PASSWORD_LEN + 1);
        let result = validate_password(&long_password);
        assert!(matches!(result, Err(PasswordError::InvalidLength { .. })));
    }

    #[test]
    fn test_missing_uppercase() {
        assert!(matches!(
            validate_password("password123"),
            Err(PasswordError::MissingUppercase)
        ));
    }

    #[test]
    fn test_missing_lowercase() {
        assert!(matches!(
            validate_password("PASSWORD123"),
            Err(PasswordError::MissingLowercase)
        ));
    }

    #[test]
    fn test_missing_digit() {
        assert!(matches!(
            validate_password("PasswordABC"),
            Err(PasswordError::MissingDigit)
        ));
    }

    #[test]
    fn test_min_boundary_length() {
        let min_password = "a".repeat(MIN_PASSWORD_LEN - 1);
        assert!(validate_password(&min_password).is_err());

        let valid_min = "aB1".repeat(3);
        assert!(validate_password(&valid_min).is_ok());
    }
}
