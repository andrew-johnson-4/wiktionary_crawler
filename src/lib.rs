use serde_with::skip_serializing_none;
use serde::{Serialize,Deserialize};

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct WiktionaryEntry {
   word: String,
   language: Option<String>,
   ipa_pronunciation: Option<Vec<String>>,
}
