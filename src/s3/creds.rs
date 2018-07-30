use futures::{
    future::{result, FutureResult}, Future, Poll,
};
use rusoto_core::{AwsCredentials, CredentialsError, ProvideAwsCredentials, Region};
use std::env;

const AWS_ACCESS_KEY_ID: &str = "REDIRECTS_AWS_ACCESS_KEY_ID";
const AWS_SECRET_ACCESS_KEY: &str = "REDIRECTS_AWS_SECRET_ACCESS_KEY";
const S3_BUCKET: &str = "REDIRECTS_S3_BUCKET";
const S3_REGION: &str = "REDIRECTS_S3_REGION";

pub fn bucket() -> Result<String, CredentialsError> {
    match get_env(S3_BUCKET) {
        Some(bucket) => Ok(bucket),
        None => Err(missing_env_err(S3_BUCKET)),
    }
}

pub fn region() -> Region {
    get_env(S3_REGION).map_or_else(Region::default, |reg| {
        reg.parse().ok().unwrap_or_else(Region::default)
    })
}

pub struct CredentialsProvider;

impl ProvideAwsCredentials for CredentialsProvider {
    type Future = CredentialsProviderFuture;

    fn credentials(&self) -> Self::Future {
        CredentialsProviderFuture {
            inner: result(get_env_creds()),
        }
    }
}

pub struct CredentialsProviderFuture {
    inner: FutureResult<AwsCredentials, CredentialsError>,
}

impl Future for CredentialsProviderFuture {
    type Item = AwsCredentials;
    type Error = CredentialsError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

fn get_env_creds() -> Result<AwsCredentials, CredentialsError> {
    let key = match get_env(AWS_ACCESS_KEY_ID) {
        Some(key) => key,
        None => return Err(missing_env_err(AWS_ACCESS_KEY_ID)),
    };

    let secret = match get_env(AWS_SECRET_ACCESS_KEY) {
        Some(secret) => secret,
        None => return Err(missing_env_err(AWS_SECRET_ACCESS_KEY)),
    };

    Ok(AwsCredentials::new(key, secret, None, None))
}

fn get_env(var_name: &str) -> Option<String> {
    match env::var(var_name) {
        Ok(value) => Some(value).filter(|str| !str.is_empty()),
        Err(_) => None,
    }
}

fn missing_env_err(var_name: &str) -> CredentialsError {
    let err = format!("Need to set env var `{}`", var_name);
    CredentialsError::new(err)
}
