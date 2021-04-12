use clap::Clap;
use serde_json;
use shopify::client::Client;
use std::env::var;

#[derive(Clap)]
struct Opts {
  #[clap(short, long, default_value = ".env")]
  config: String,
  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
  ProductList,
  VariantList,
  OrderGet(OrderGet),
}

#[derive(Clap)]
struct OrderGet {
  id: i64,
}

fn main() {
  let opts: Opts = Opts::parse();

  dotenv::from_filename(&opts.config).unwrap();
  let client = Client::new(
    &var("SHOPIFY_BASE_URL").unwrap(),
    &var("SHOPIFY_API_KEY").unwrap(),
    &var("SHOPIFY_PASSWORD").unwrap(),
  )
  .unwrap();

  match opts.subcmd {
    SubCommand::ProductList => product_list(&client),
    SubCommand::VariantList => variant_list(&client),
    SubCommand::OrderGet(OrderGet { id }) => order_get(&client, id),
  }
}

fn product_list(client: &Client) {
  use shopify::product::*;

  let mut all = vec![];

  let page = client
    .list(&GetProductListParams {
      ..Default::default()
    })
    .unwrap();

  let mut next = page.get_next();

  all.extend(page.payload);

  loop {
    if let Some(params) = next {
      let page = client.list_page(&params).unwrap();
      next = page.get_next();

      all.extend(page.payload);
    } else {
      break;
    }
  }

  serde_json::to_writer_pretty(std::io::stdout(), &all).unwrap()
}

fn variant_list(client: &Client) {
  use shopify::variant::*;

  let mut all = vec![];

  let page = client
    .list(&GetVariantListParams {
      ..Default::default()
    })
    .unwrap();

  let mut next = page.get_next();

  all.extend(page.payload);

  loop {
    if let Some(params) = next {
      let page = client.list_page(&params).unwrap();
      next = page.get_next();

      all.extend(page.payload);
    } else {
      break;
    }
  }

  serde_json::to_writer_pretty(std::io::stdout(), &all).unwrap()
}

fn order_get(client: &Client, id: i64) {
  use shopify::order::*;

  let order = client.get(id).unwrap();
  serde_json::to_writer_pretty(std::io::stdout(), &order).unwrap()
}
