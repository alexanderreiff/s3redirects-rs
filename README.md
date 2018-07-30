# S3Redirects-rs

A Rust application that generates a Nginx configuration file from a CSV list
of redirect rules stored in Amazon S3.

## Input File Format
The redirect rules are expected to be in a CSV format with headers
`match_pattern` and `redirect_pattern`.

Example:
```csv
match_pattern,redirect_pattern
^/resources/(.+)/subs(/.*)?$,/new-resources/$1/new-sub$2
^/simple,/short
```

## AWS Authentication and API Access
The app assumes the following environment variables are set in order to access
the source file in AWS S3:

* REDIRECTS_AWS_ACCESS_KEY_ID
* REDIRECTS_AWS_SECRET_ACCESS_KEY
* REDIRECTS_S3_BUCKET
* REDIRECTS_S3_REGION

## Usage
```
s3redirects --out my/path/to/out.conf --in my/rules.csv --etag "xxxxxxxxx"
```

* `--out` (required) path to write Nginx conf; will overwrite existing file
* `--in` S3 file key; defaults to `redirect_rules/latest.csv`
* `--etag` ETag to compare against file stored in S3; if matching, the app will
exit with status code 3
