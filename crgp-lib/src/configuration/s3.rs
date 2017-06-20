// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Configuration for AWS S3 access.

use std::fmt;

use s3::bucket::Bucket;
use s3::credentials::Credentials;
use s3::region::Region;

use Result;
use aws_s3::credentials_from_env;

/// Configuration for accessing AWS S3. The access and secret key will be loaded from respective environment variables
/// when requesting the bucket.
///
/// Neither the access key nor the secret key will ever be written when serializing the S3 configuration!
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct S3 {
    /// The bucket to access.
    pub bucket: String,

    /// The AWS region where the bucket is located.
    pub region: String,

    /// Private field to prevent initialization without the provided methods.
    ///
    /// All other fields should be public for easy access without getter functions. However, adding more fields later
    /// could break code if the `S3Configuration` were manually initialized.
    #[serde(skip_serializing)]
    _prevent_outside_initialization: bool,
}

impl S3 {
    /// Initialize a configuration for accessing AWS S3.
    pub fn new(bucket: &str, region: &str) -> S3 {
        S3 {
            bucket: String::from(bucket),
            region: String::from(region),
            _prevent_outside_initialization: true,
        }
    }

    /// Get a connection to AWS S3.
    pub fn get_bucket(&self) -> Result<Bucket> {
        let credentials: Credentials = credentials_from_env()?;
        let region: Region = self.region.parse()?;
        Ok(Bucket::new(&self.bucket, region, credentials))
    }
}

impl fmt::Display for S3 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{bucket} ({region})", bucket = self.bucket, region = self.region)
    }
}

#[cfg(test)]
mod tests {
    use std::env::remove_var;
    use std::env::set_var;
    use s3::bucket::Bucket;
    use s3::region::Region;
    use Result;
    use super::*;


    /// The name of the environment variable with the AWS access key ID.
    const ACCESS_KEY_VAR_NAME: &str = "AWS_ACCESS_KEY_ID";

    /// The name of the environment variable with the AWS secret access key.
    const SECRET_VAR_NAME: &str = "AWS_SECRET_ACCESS_KEY";

    #[test]
    fn new() {
        let s3 = S3::new("bucket", "region");
        assert_eq!(s3.bucket, String::from("bucket"));
        assert_eq!(s3.region, String::from("region"));
        assert!(s3._prevent_outside_initialization);
    }

    #[test]
    fn get_bucket_success() {
        let bucket_name: &str = "bucket";
        let region = Region::UsEast1;
        let access_key_id: &str = "Access Key ID";
        let secret_access_key: &str = "Secret Access Key";
        set_var(ACCESS_KEY_VAR_NAME, access_key_id);
        set_var(SECRET_VAR_NAME, secret_access_key);

        let s3 = S3::new(bucket_name, &format!("{}", region));
        let bucket: Result<Bucket> = s3.get_bucket();
        assert!(bucket.is_ok());
        let bucket: Bucket = bucket.unwrap();
        assert_eq!(bucket.name, String::from(bucket_name));
        assert_eq!(bucket.region, region);
        assert_eq!(bucket.credentials.access_key, String::from(access_key_id));
        assert_eq!(bucket.credentials.secret_key, String::from(secret_access_key));
        remove_var(ACCESS_KEY_VAR_NAME);
        remove_var(SECRET_VAR_NAME);
    }

    #[test]
    fn get_bucket_failure_env_vars() {
        let bucket_name: &str = "bucket";
        let region = Region::UsEast1;
        let secret_access_key: &str = "Secret Access Key";

        let s3 = S3::new(bucket_name, &format!("{}", region));
        remove_var(ACCESS_KEY_VAR_NAME);
        set_var(SECRET_VAR_NAME, secret_access_key);
        let bucket: Result<Bucket> = s3.get_bucket();
        assert!(bucket.is_err());
        remove_var(SECRET_VAR_NAME);
    }

    #[test]
    fn get_bucket_failure_region() {
        let bucket_name: &str = "bucket";
        let region: &str = "test-region-that-should-not-exist";
        let access_key_id: &str = "Access Key ID";
        let secret_access_key: &str = "Secret Access Key";

        let s3 = S3::new(bucket_name, region);
        set_var(ACCESS_KEY_VAR_NAME, access_key_id);
        set_var(SECRET_VAR_NAME, secret_access_key);
        let bucket: Result<Bucket> = s3.get_bucket();
        assert!(bucket.is_err());
        remove_var(ACCESS_KEY_VAR_NAME);
        remove_var(SECRET_VAR_NAME);
    }

    #[test]
    fn fmt_display() {
        let s3 = S3::new("bucket", "region");
        assert_eq!(format!("{}", s3), String::from("bucket (region)"));
    }
}

