#[doc(hidden)]
pub trait ShopifyWarpper<T> {
  fn into_inner(self) -> T;
}

#[doc(hidden)]
pub trait ShopifyRequestQuery {
  fn as_query_pairs(&self) -> Vec<(String, String)>;
}

impl ShopifyRequestQuery for () {
  fn as_query_pairs(&self) -> Vec<(String, String)> {
    vec![]
  }
}

impl<T> ShopifyRequestQuery for Option<T>
where
  T: ShopifyRequestQuery,
{
  fn as_query_pairs(&self) -> Vec<(String, String)> {
    match *self {
      Some(ref v) => v.as_query_pairs(),
      None => vec![],
    }
  }
}

impl<K, V> ShopifyRequestQuery for (K, V)
where
  K: AsRef<str>,
  V: AsRef<str>,
{
  fn as_query_pairs(&self) -> Vec<(String, String)> {
    vec![(self.0.as_ref().to_owned(), self.1.as_ref().to_owned())]
  }
}
