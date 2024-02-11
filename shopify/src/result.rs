use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShopifyError {
  #[error("not found")]
  NotFound,

  #[error("request error: path = '{path}', status = '{status}', body = '{body}'")]
  Request {
    path: String,
    status: StatusCode,
    body: String,
  },

  #[error("invalid response")]
  InvalidResponse,

  #[error("http error: {0}")]
  Http(#[from] reqwest::Error),

  #[error("io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),

  #[error("url parse error: {0}")]
  UrlParse(#[from] url::ParseError),

  #[error("page_info parameter was not found in the link url")]
  PageInfoNotPresent,
}

impl ShopifyError {
  pub fn should_try_again(&self) -> bool {
    match *self {
      ShopifyError::Request { status, .. } => {
        let code = status.as_u16();
        // 429 Too Many Requests
        code == 429 || code == 500 || code == 503
      }
      ShopifyError::Io(_) => true,
      _ => false,
    }
  }
}

pub type ShopifyResult<T> = Result<T, ShopifyError>;

pub trait OptionalShopifyResult<T> {
  fn optional(self) -> ShopifyResult<Option<T>>;
}

impl<T> OptionalShopifyResult<T> for ShopifyResult<T> {
  fn optional(self) -> ShopifyResult<Option<T>> {
    match self {
      Ok(v) => Ok(Some(v)),
      Err(ShopifyError::NotFound) => Ok(None),
      Err(e) => Err(e),
    }
  }
}
