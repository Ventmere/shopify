use reqwest::{Client as HttpClient, RequestBuilder, StatusCode, Url};
use types::{DateTime, Utc};
use error::*;
pub use reqwest::Method;
use serde::Deserialize;

mod types;
pub use self::types::*;

#[doc(hidden)]
#[macro_export]
macro_rules! shopify_wrap {
  (
    pub struct $t:ident {
      $key:ident: $inner_t:ty$(,)*
    }
  ) => {
    use client::ShopifyWarpper;

    #[derive(Debug, Deserialize)]
    pub struct $t {
      $key: $inner_t,
    }

    impl ShopifyWarpper<$inner_t> for $t {
      fn into_inner(self) -> $inner_t {
        self.$key
      }
    }
  }
}

pub(crate) trait AsQueryValue {
  fn as_query_value(&self) -> String;
}

macro_rules! impl_into_query_value {
  (
    $t:ty
  ) => (
    impl AsQueryValue for $t {
      fn as_query_value(&self) -> String {
        format!("{}", self)
      }
    }
  );

  (
    $t:ty, $f:expr
  ) => (
    impl AsQueryValue for $t {
      fn as_query_value(&self) -> String {
        $f(self)
      }
    }
  );
}

impl_into_query_value!(i64);
impl_into_query_value!(String);
impl_into_query_value!(DateTime<Utc>, |date: &DateTime<Utc>| date.to_rfc3339());

impl<T> AsQueryValue for Vec<T>
where
  T: AsQueryValue,
{
  fn as_query_value(&self) -> String {
    self
      .iter()
      .map(|v| v.as_query_value())
      .collect::<Vec<_>>()
      .join(",")
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! request_query {
  (
    pub struct $t:ident {
      $(pub $key:ident: Option<$field_t:ty>),*
      $(,)*
    }
  ) => (
    use client::{ShopifyRequestQuery, AsQueryValue};

    #[derive(Debug, Default)]
    pub struct $t {
      $(
        pub $key: Option<$field_t>
      ),*
    }

    impl ShopifyRequestQuery for $t {
      fn as_query_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = vec![];
        $(
          request_query!(PUSH_FIELD self, pairs, $key, $field_t);
        )*
        pairs
      }
    }
  );

  (PUSH_FIELD $s:expr, $v:expr, $key:ident, $t:tt) => {
    if let Some(ref v) = $s.$key {
      $v.push((
        stringify!($key).to_owned(),
        v.as_query_value(),
      ));
    }
  };
}

#[derive(Debug, Clone)]
pub struct Client {
  base_url: Url,
  api_key: String,
  password: String,
  client: HttpClient,
}

impl Client {
  pub fn new(base_url: &str, api_key: &str, password: &str) -> Result<Self> {
    Ok(Client {
      base_url: Url::parse(base_url)?,
      api_key: api_key.to_owned(),
      password: password.to_owned(),
      client: HttpClient::new(),
    })
  }

  pub fn with_http_client(
    client: HttpClient,
    base_url: &str,
    api_key: &str,
    password: &str,
  ) -> Result<Self> {
    Ok(Client {
      base_url: Url::parse(base_url)?,
      api_key: api_key.to_owned(),
      password: password.to_owned(),
      client: client,
    })
  }

  pub fn request_with_params<P, T, F>(
    &self,
    method: Method,
    path: &str,
    params: &P,
    bf: F,
  ) -> Result<T>
  where
    P: ShopifyRequestQuery,
    T: for<'de> Deserialize<'de>,
    F: FnOnce(&mut RequestBuilder),
  {
    let mut url = self.base_url.join(path)?;
    url.query_pairs_mut().extend_pairs(params.as_query_pairs());
    let mut b = self.client.request(method, url);
    b.basic_auth(self.api_key.clone(), Some(self.password.clone()));

    bf(&mut b);

    let mut res = b.send()?;

    if !res.status().is_success() {
      if res.status() == StatusCode::NotFound {
        return Err(ErrorKind::NotFound.into());
      }

      let body = res.text()?;
      return Err(ErrorKind::Request(path.to_owned(), res.status(), body).into());
    }

    res.json().chain_err(|| ErrorKind::InvalidResponse)
  }

  pub fn request<T, F>(&self, method: Method, path: &str, bf: F) -> Result<T>
  where
    T: for<'de> Deserialize<'de>,
    F: FnOnce(&mut RequestBuilder),
  {
    self.request_with_params(method, path, &(), bf)
  }
}

#[cfg(test)]
pub fn get_test_client() -> Client {
  use std::env::var;
  ::dotenv::dotenv().ok();

  Client::new(
    &var("SHOPIFY_BASE_URL").unwrap(),
    &var("SHOPIFY_API_KEY").unwrap(),
    &var("SHOPIFY_PASSWORD").unwrap(),
  ).unwrap()
}
