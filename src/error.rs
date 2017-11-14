use reqwest::StatusCode;

error_chain! {
  errors {
    NotFound

    Request(path: String, status: StatusCode, body: String) {
      description("request error")
      display("request error: path = '{}', status = '{}', body = '{}'", path, status, body)
    }

    InvalidResponse
  }

  foreign_links {
    Http(::reqwest::Error);
    Url(::reqwest::UrlError);
    Io(::std::io::Error);
    Json(::serde_json::Error);
  }
}

pub trait OptionalResult<T> {
  fn optional(self) -> Result<Option<T>>;
}

impl<T> OptionalResult<T> for Result<T> {
  fn optional(self) -> Result<Option<T>> {
    match self {
      Ok(v) => Ok(Some(v)),
      Err(Error(ErrorKind::NotFound, _)) => Ok(None),
      Err(e) => Err(e),
    }
  }
}