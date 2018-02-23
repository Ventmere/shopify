use error::*;
use client::{Client, Method};
use types::Value;
use serde::Serialize;

mod types;
pub use self::types::*;

pub struct ProductVariantApi<'a> {
  client: &'a Client,
}

impl Client {
  pub fn product_variant(&self) -> ProductVariantApi {
    ProductVariantApi { client: self }
  }
}

request_query! {
  pub struct GetVariantListParams {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub since_id: Option<i64>,
    pub fields: Option<Vec<String>>,
  }
}

impl<'a> ProductVariantApi<'a> {
  pub fn get_list(&self, params: &GetVariantListParams) -> Result<Vec<Variant>> {
    shopify_wrap! {
      pub struct Res {
        varaints: Vec<Variant>,
      }
    }

    let res: Res =
      self
        .client
        .request_with_params(Method::Get, "/admin/variants.json", params, |_| {})?;
    Ok(res.into_inner())
  }

  pub fn update<V: Serialize>(&self, id: i64, value: V) -> Result<Variant> {
    shopify_wrap! {
      pub struct Res {
        varaint: Variant,
      }
    }

    let path = format!("/admin/variants/{}.json", id);
    let res: Res = self.client.request(Method::Put, &path, move |b| {
      b.json(&json!({
        "variant": value,
      }));
      ()
    })?;
    Ok(res.into_inner())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const TMP_DIR: &'static str = "./tmp/variants";

  #[test]
  fn test_dump_all_variants() {
    use std::fs::File;
    use std::io::Write;
    use serde_json::{self, Value};
    use chrono::TimeZone;

    shopify_wrap! {
      pub struct RawVariants {
        variants: Vec<Value>,
      }
    }

    let client = ::client::get_test_client();
    let mut params = GetVariantListParams::default();
    params.limit = Some(250);
    let mut page = 1;
    loop {
      println!("Downloading page {}", page);

      let variants = client
        .request_with_params::<_, RawVariants, _>(
          Method::Get,
          "/admin/variants.json",
          &params,
          |_| {},
        )
        .unwrap()
        .into_inner();

      if variants.is_empty() {
        break;
      }

      let id = variants
        .last()
        .unwrap()
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap();
      println!("count = {}, last_id = {}", variants.len(), id);

      params.page = Some(page + 1);

      let f = File::create(format!("{}/variant_{}.json", TMP_DIR, page)).unwrap();
      serde_json::to_writer_pretty(f, &variants).unwrap();

      page = page + 1;
      ::std::thread::sleep_ms(500);
    }
  }

  #[test]
  // #[ignore]
  fn test_deserialize_all_variants() {
    use std::io;
    use std::io::Write;
    use std::fs;
    use serde_json::{self, Value};

    let mut chunk = 1;
    loop {
      let f = match fs::File::open(format!("{}/variant_{}.json", TMP_DIR, chunk)) {
        Ok(f) => f,
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
          break;
        }
        Err(ref err) => panic!("io err: {}", err),
      };

      println!("testing chunk {}", chunk);

      let variants: Vec<Value> = serde_json::from_reader(f).unwrap();

      let total = variants.len();
      for (i, variant) in variants.into_iter().enumerate() {
        let id = variant.get("id").unwrap().as_i64().unwrap();
        let as_str = serde_json::to_string_pretty(&variant).unwrap();
        let mut current = fs::File::create(format!("{}/current_variant.json", TMP_DIR)).unwrap();
        write!(&mut current, "{}", &as_str).unwrap();
        let variant: Variant = serde_json::from_str(&as_str).unwrap();
        println!("testing variant {}: {} of {}", id, i + 1, total);
      }

      chunk = chunk + 1;
    }
  }
}
