use futures::{stream::Stream, Future};
use rusoto_core::Region;
use rusoto_s3::{GetObjectError, GetObjectRequest, S3, S3Client, StreamingBody};
use std::error::Error;
use std::io::Cursor;

pub struct File {
    etag: Option<String>,
    bytes: Vec<u8>,
}

impl File {
    pub fn into_reader(self) -> Cursor<Box<[u8]>> {
        Cursor::new(self.bytes.into_boxed_slice())
    }

    pub fn etag(&self) -> Option<String> {
        self.etag.clone()
    }
}

#[derive(Debug)]
pub enum FileError {
    NotFound,
    NotModified,
    Unknown(Box<Error>),
}

pub fn fetch(etag: Option<&str>) -> Result<File, FileError> {
    // TODO: Region env var
    let client = S3Client::simple(Region::UsWest2);
    let request = build_s3_request(etag);
    let response = client.get_object(&request).sync();

    if let Err(err) = response {
        return Err(normalize_s3_error(err, etag));
    }

    let response = response.unwrap();

    if response.body.is_none() {
        return Err(FileError::Unknown(From::from("File empty")));
    }

    let stream = read_s3_body(response.body);

    if let Err(err) = stream {
        return Err(err);
    }

    Ok(File {
        bytes: stream.unwrap(),
        etag: response.e_tag,
    })
}

fn build_s3_request(etag: Option<&str>) -> GetObjectRequest {
    let if_none_match = match etag {
        Some(etag) => Some(etag.to_string()),
        None => None,
    };

    GetObjectRequest {
        if_none_match,
        bucket: "ar-redirects-test".to_string(),
        key: "latest.csv".to_string(),
        ..Default::default()
    }
}

fn read_s3_body(body: Option<StreamingBody>) -> Result<Vec<u8>, FileError> {
    if body.is_none() {
        return Err(FileError::Unknown(From::from("File empty")));
    }

    match body.unwrap().concat2().wait() {
        Ok(bytes) => Ok(bytes),
        Err(err) => Err(FileError::Unknown(From::from(err))),
    }
}

fn normalize_s3_error(err: GetObjectError, etag: Option<&str>) -> FileError {
    match err {
        GetObjectError::NoSuchKey(_) => FileError::NotFound,
        GetObjectError::HttpDispatch(err) => FileError::Unknown(From::from(err)),
        GetObjectError::Credentials(err) => FileError::Unknown(From::from(err)),
        GetObjectError::Validation(err) => FileError::Unknown(From::from(err)),
        GetObjectError::Unknown(err) => {
            match etag {
                // If body is empty and unknown error has occurred when supplying `If-None-Match`
                // HTTP header, it is safe to assume a 304 Not Modified response was returned.
                // These are returned with empty responses from S3.
                // rusoto needs better handling of HTTP headers.
                Some(_) => FileError::NotModified,
                None => FileError::Unknown(From::from(err)),
            }
        }
    }
}
