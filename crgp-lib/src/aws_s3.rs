// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Convenience module for more simple AWS S3 access.

/// The name of the environment variable with the AWS access key ID.
pub const ACCESS_KEY_VAR_NAME: &str = "AWS_ACCESS_KEY_ID";

/// The name of the environment variable with the AWS secret access key.
pub const SECRET_VAR_NAME: &str = "AWS_SECRET_ACCESS_KEY";

/// The name of the environment variable with the AWS token.
pub const TOKEN_VAR_NAME: &str = "AWS_TOKEN";

use std::env::var;

use s3::credentials::Credentials;

use Result;

/// Load the access key ID and the secret access key for AWS S3 from respective environment variables.
///
/// Required environment variables:
///
///  * `AWS_ACCESS_KEY_ID`
///  * `AWS_SECRET_ACCESS_KEY`
///
/// Optional environment variables:
///
///  * `AWS_TOKEN`
///
/// Return an error if required environment variables are missing.
pub fn credentials_from_env() -> Result<Credentials> {
    // Get the environment variables.
    let access_key_id: String = var(ACCESS_KEY_VAR_NAME)?;
    let secret_access_key: String = var(SECRET_VAR_NAME)?;
    let token: Option<String> = var(TOKEN_VAR_NAME).ok();

    // Build the credentials. To avoid `String` to `&str` conversion with nasty lifetimes, set it manually.
    let mut credentials = Credentials::new(&access_key_id, &secret_access_key, None);
    credentials.token = token;
    Ok(credentials)
}

#[cfg(test)]
mod tests {
    use std::env::remove_var;
    use std::env::set_var;
    use s3::credentials::Credentials;
    use Result;
    use super::*;

    #[test]
    fn credentials_from_env() {
        let access_key_id: &str = "Access Key ID";
        let secret_access_key: &str = "Secret Access Key";
        let token: &str = "Token";

        // Ensure there are no variables set when testing.
        remove_var(ACCESS_KEY_VAR_NAME);
        remove_var(SECRET_VAR_NAME);
        remove_var(TOKEN_VAR_NAME);

        // No environment variables set.
        let credentials: Result<Credentials> = super::credentials_from_env();
        assert!(credentials.is_err());

        // Only one required variable is set.
        set_var(ACCESS_KEY_VAR_NAME, access_key_id);
        let credentials: Result<Credentials> = super::credentials_from_env();
        assert!(credentials.is_err());
        remove_var(ACCESS_KEY_VAR_NAME);

        set_var(SECRET_VAR_NAME, secret_access_key);
        let credentials: Result<Credentials> = super::credentials_from_env();
        assert!(credentials.is_err());
        remove_var(SECRET_VAR_NAME);

        // Both required variables are set, no optional one.
        set_var(ACCESS_KEY_VAR_NAME, access_key_id);
        set_var(SECRET_VAR_NAME, secret_access_key);
        let credentials: Result<Credentials> = super::credentials_from_env();
        assert!(credentials.is_ok());
        let credentials: Credentials = credentials.unwrap();
        assert_eq!(credentials.access_key, String::from(access_key_id));
        assert_eq!(credentials.secret_key, String::from(secret_access_key));
        assert_eq!(credentials.token, None);
        remove_var(ACCESS_KEY_VAR_NAME);
        remove_var(SECRET_VAR_NAME);

        // All variables are set.
        set_var(ACCESS_KEY_VAR_NAME, access_key_id);
        set_var(SECRET_VAR_NAME, secret_access_key);
        set_var(TOKEN_VAR_NAME, token);
        let credentials: Result<Credentials> = super::credentials_from_env();
        assert!(credentials.is_ok());
        let credentials: Credentials = credentials.unwrap();
        assert_eq!(credentials.access_key, String::from(access_key_id));
        assert_eq!(credentials.secret_key, String::from(secret_access_key));
        assert_eq!(credentials.token, Some(String::from(token)));
        remove_var(ACCESS_KEY_VAR_NAME);
        remove_var(SECRET_VAR_NAME);
        remove_var(TOKEN_VAR_NAME);
    }
}
