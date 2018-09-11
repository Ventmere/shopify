use reqwest::StatusCode;

#[derive(Fail, Debug)]
pub enum ShopifyError {
  #[fail(display = "not found")]
  NotFound,

  #[fail(
    display = "request error: path = '{}', status = '{}', body = '{}'",
    path,
    status,
    body
  )]
  Request {
    path: String,
    status: StatusCode,
    body: String,
  },

  #[fail(display = "invalid response")]
  InvalidResponse,

  #[fail(display = "http error: {}", _0)]
  Http(::reqwest::Error),

  #[fail(display = "url error: {}", _0)]
  Url(::reqwest::UrlError),

  #[fail(display = "io error: {}", _0)]
  Io(::std::io::Error),

  #[fail(display = "json error: {}", _0)]
  Json(::serde_json::Error),
}

impl ShopifyError {
  pub fn should_try_again(&self) -> bool {
    match *self {
      ShopifyError::Request { status, .. } => {
        let code = status.as_u16();
        // 429 Too Many Requests
        code == 429 || code == 500 || code == 503
      }
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

macro_rules! impl_from {
  ($v:ident($t:ty)) => {
    impl From<$t> for ShopifyError {
      fn from(e: $t) -> Self {
        ShopifyError::$v(e)
      }
    }
  };
}

impl_from!(Http(::reqwest::Error));
impl_from!(Url(::reqwest::UrlError));
impl_from!(Io(::std::io::Error));
impl_from!(Json(::serde_json::Error));
