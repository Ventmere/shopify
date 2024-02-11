use crate::pagination::Paginated;
use crate::result::*;
use crate::types::{DateTime, Utc};
use reqwest::blocking::Response;
pub use reqwest::Method;
use reqwest::{blocking::Client as HttpClient, blocking::RequestBuilder, StatusCode, Url};
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
    use $crate::client::ShopifyWrapper;

    #[derive(Debug, Deserialize)]
    pub struct $t {
      $key: $inner_t,
    }

    impl ShopifyWrapper<$inner_t> for $t {
      fn into_inner(self) -> $inner_t {
        self.$key
      }
    }
  };
}

pub(crate) trait AsQueryValue {
  fn as_query_value(&self) -> String;
}

macro_rules! impl_into_query_value {
  (
    $t:ty
  ) => {
    impl AsQueryValue for $t {
      fn as_query_value(&self) -> String {
        format!("{}", self)
      }
    }
  };

  (
    $t:ty, $f:expr
  ) => {
    impl AsQueryValue for $t {
      fn as_query_value(&self) -> String {
        $f(self)
      }
    }
  };
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
    use $crate::client::{ShopifyRequestQuery, AsQueryValue};

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
  pub fn new(base_url: &str, api_key: &str, password: &str) -> ShopifyResult<Self> {
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
  ) -> ShopifyResult<Self> {
    Ok(Client {
      base_url: Url::parse(base_url)?,
      api_key: api_key.to_owned(),
      password: password.to_owned(),
      client,
    })
  }

  pub fn request_with_params<P, T, F>(
    &self,
    method: Method,
    path: &str,
    params: &P,
    bf: F,
  ) -> ShopifyResult<T>
  where
    P: ShopifyRequestQuery,
    T: for<'de> Deserialize<'de>,
    F: FnOnce(RequestBuilder) -> RequestBuilder,
  {
    let mut url = self.base_url.join(path)?;
    url.query_pairs_mut().extend_pairs(params.as_query_pairs());
    let mut b = self.client.request(method, url);
    b = b.basic_auth(self.api_key.clone(), Some(self.password.clone()));

    b = bf(b);

    let res = b.send()?;
    let status = res.status();
    if !status.is_success() {
      if status == StatusCode::NOT_FOUND {
        return Err(ShopifyError::NotFound);
      }

      let body = res.text()?;

      return Err(ShopifyError::Request {
        path: path.to_owned(),
        status,
        body,
      });
    }

    res.json().map_err(Into::into)
  }

  pub fn request_with_params_paginated<P, T, F>(
    &self,
    method: Method,
    path: &str,
    params: &P,
    bf: F,
  ) -> ShopifyResult<Paginated<T>>
  where
    P: ShopifyRequestQuery,
    T: for<'de> Deserialize<'de>,
    F: FnOnce(RequestBuilder) -> RequestBuilder,
  {
    let mut url = self.base_url.join(path)?;
    url.query_pairs_mut().extend_pairs(params.as_query_pairs());
    let mut b = self.client.request(method, url);
    b = b.basic_auth(self.api_key.clone(), Some(self.password.clone()));

    b = bf(b);

    let res = b.send()?;
    let status = res.status();

    if !status.is_success() {
      if status == StatusCode::NOT_FOUND {
        return Err(ShopifyError::NotFound);
      }

      let body = res.text()?;
      return Err(ShopifyError::Request {
        path: path.to_owned(),
        status,
        body,
      });
    }

    Paginated::from_res(res)
  }

  pub fn request<T, F>(&self, method: Method, path: &str, bf: F) -> ShopifyResult<T>
  where
    T: for<'de> Deserialize<'de>,
    F: FnOnce(RequestBuilder) -> RequestBuilder,
  {
    self.request_with_params(method, path, &(), bf)
  }

  pub fn request_paginated<T, F>(
    &self,
    method: Method,
    path: &str,
    bf: F,
  ) -> ShopifyResult<Paginated<T>>
  where
    T: for<'de> Deserialize<'de>,
    F: FnOnce(RequestBuilder) -> RequestBuilder,
  {
    self.request_with_params_paginated(method, path, &(), bf)
  }

  pub fn request_raw<F>(&self, method: Method, path: &str, bf: F) -> ShopifyResult<Response>
  where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
  {
    let url = self.base_url.join(path)?;
    let mut b = self.client.request(method, url);
    b = b.basic_auth(self.api_key.clone(), Some(self.password.clone()));

    b = bf(b);

    b.send().map_err(Into::into)
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
  )
  .unwrap()
}
