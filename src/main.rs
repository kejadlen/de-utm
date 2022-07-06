use std::env;

use anyhow::{anyhow, Result};
use url::Url;

fn main() -> Result<()> {
    let url = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("no url provided"))?;

    let url = Url::parse(&url)?;
    let no_utm: Vec<(_, _)> = url
        .query_pairs()
        .filter(|(k, _)| !k.starts_with("utm_"))
        .collect();

    let mut url = url.clone();
    if no_utm.is_empty() {
        url.set_query(None);
    } else {
        url.query_pairs_mut().clear().extend_pairs(no_utm);
    }

    print!("{}", &url);

    Ok(())
}
