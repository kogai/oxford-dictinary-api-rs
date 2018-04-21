use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T>
where
    T: serde::Deserialize<'de>,
{
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError {
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError {
                code: e.0,
                content: Some(t),
            }),
            Err(e) => Error::from(e),
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e);
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

use super::models::*;

mod dictionary_entries_api;
pub use self::dictionary_entries_api::{DictionaryEntriesApi, DictionaryEntriesApiClient};
mod lemmatron_api;
pub use self::lemmatron_api::{LemmatronApi, LemmatronApiClient};
mod lexi_stats_api;
pub use self::lexi_stats_api::{LexiStatsApi, LexiStatsApiClient};
mod search_api;
pub use self::search_api::{SearchApi, SearchApiClient};
mod the_sentence_dictionary_api;
pub use self::the_sentence_dictionary_api::{TheSentenceDictionaryApi,
                                            TheSentenceDictionaryApiClient};
mod thesaurus_api;
pub use self::thesaurus_api::{ThesaurusApi, ThesaurusApiClient};
mod translation_api;
pub use self::translation_api::{TranslationApi, TranslationApiClient};
mod utility_api;
pub use self::utility_api::{UtilityApi, UtilityApiClient};
mod wordlist_api;
pub use self::wordlist_api::{WordlistApi, WordlistApiClient};

pub mod configuration;
pub mod client;
