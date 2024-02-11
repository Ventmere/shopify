use crate::client::ShopifyRequestQuery;
use crate::result::{ShopifyError, ShopifyResult};
use reqwest::blocking::Response;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Default, Clone)]
pub struct GetPage {
  pub page_info: String,
  pub limit: Option<i64>,
  pub fields: Option<Vec<String>>,
}

impl GetPage {
  pub fn limit(self, limit: i64) -> Self {
    Self {
      limit: Some(limit),
      ..self
    }
  }
}

impl ShopifyRequestQuery for GetPage {
  fn as_query_pairs(&self) -> Vec<(String, String)> {
    let mut pairs = vec![("page_info".to_string(), self.page_info.clone())];

    if let Some(limit) = self.limit.clone() {
      pairs.push(("limit".to_string(), limit.to_string()))
    }

    if let Some(fields) = self.fields.as_ref() {
      pairs.push(("fields".to_string(), fields.join(",")))
    }
    pairs
  }
}

#[derive(Debug)]
pub struct Paginated<T> {
  pub payload: T,
  pub previous_page_info: Option<String>,
  pub next_page_info: Option<String>,
}

impl<T> Paginated<T> {
  pub fn from_res(res: Response) -> ShopifyResult<Paginated<T>>
  where
    T: for<'de> Deserialize<'de>,
  {
    let mut previous_url = None;
    let mut next_url = None;
    if let Some(link) = res.headers().get("link").and_then(|v| {
      v.to_str()
        .ok()
        .and_then(|v| parse_link_header::parse(v).ok())
    }) {
      for (k, v) in link {
        if let Some(rel) = k {
          if rel == "prev" {
            previous_url = v.raw_uri.into();
          } else if rel == "next" {
            next_url = v.raw_uri.into();
          }
        }
      }
    }
    Ok(Paginated {
      previous_page_info: previous_url.map(|v| parse_page_info(&v)).transpose()?,
      next_page_info: next_url.map(|v| parse_page_info(&v)).transpose()?,
      payload: res.json()?,
    })
  }

  pub fn map<F, R>(self, f: F) -> Paginated<R>
  where
    F: FnOnce(T) -> R,
  {
    Paginated {
      payload: f(self.payload),
      previous_page_info: self.previous_page_info,
      next_page_info: self.next_page_info,
    }
  }

  pub fn get_next(&self) -> Option<GetPage> {
    self.next_page_info.as_ref().map(|info| GetPage {
      page_info: info.clone(),
      ..Default::default()
    })
  }

  pub fn get_previous(&self) -> Option<GetPage> {
    self.previous_page_info.as_ref().map(|info| GetPage {
      page_info: info.clone(),
      ..Default::default()
    })
  }
}

fn parse_page_info(url: &str) -> ShopifyResult<String> {
  let parsed = Url::parse(url).map_err(ShopifyError::UrlParse)?;
  let value = parsed
    .query_pairs()
    .find(|(k, _)| k == "page_info")
    .map(|(_, v)| v.to_string())
    .ok_or_else(|| ShopifyError::PageInfoNotPresent)?;
  Ok(value)
}
