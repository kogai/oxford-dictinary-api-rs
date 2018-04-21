use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  dictionary_entries_api: Box<::apis::DictionaryEntriesApi>,
  lemmatron_api: Box<::apis::LemmatronApi>,
  lexi_stats_api: Box<::apis::LexiStatsApi>,
  search_api: Box<::apis::SearchApi>,
  the_sentence_dictionary_api: Box<::apis::TheSentenceDictionaryApi>,
  thesaurus_api: Box<::apis::ThesaurusApi>,
  translation_api: Box<::apis::TranslationApi>,
  utility_api: Box<::apis::UtilityApi>,
  wordlist_api: Box<::apis::WordlistApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      dictionary_entries_api: Box::new(::apis::DictionaryEntriesApiClient::new(rc.clone())),
      lemmatron_api: Box::new(::apis::LemmatronApiClient::new(rc.clone())),
      lexi_stats_api: Box::new(::apis::LexiStatsApiClient::new(rc.clone())),
      search_api: Box::new(::apis::SearchApiClient::new(rc.clone())),
      the_sentence_dictionary_api: Box::new(
        ::apis::TheSentenceDictionaryApiClient::new(rc.clone()),
      ),
      thesaurus_api: Box::new(::apis::ThesaurusApiClient::new(rc.clone())),
      translation_api: Box::new(::apis::TranslationApiClient::new(rc.clone())),
      utility_api: Box::new(::apis::UtilityApiClient::new(rc.clone())),
      wordlist_api: Box::new(::apis::WordlistApiClient::new(rc.clone())),
    }
  }

  pub fn dictionary_entries_api(&self) -> &::apis::DictionaryEntriesApi {
    self.dictionary_entries_api.as_ref()
  }

  pub fn lemmatron_api(&self) -> &::apis::LemmatronApi {
    self.lemmatron_api.as_ref()
  }

  pub fn lexi_stats_api(&self) -> &::apis::LexiStatsApi {
    self.lexi_stats_api.as_ref()
  }

  pub fn search_api(&self) -> &::apis::SearchApi {
    self.search_api.as_ref()
  }

  pub fn the_sentence_dictionary_api(&self) -> &::apis::TheSentenceDictionaryApi {
    self.the_sentence_dictionary_api.as_ref()
  }

  pub fn thesaurus_api(&self) -> &::apis::ThesaurusApi {
    self.thesaurus_api.as_ref()
  }

  pub fn translation_api(&self) -> &::apis::TranslationApi {
    self.translation_api.as_ref()
  }

  pub fn utility_api(&self) -> &::apis::UtilityApi {
    self.utility_api.as_ref()
  }

  pub fn wordlist_api(&self) -> &::apis::WordlistApi {
    self.wordlist_api.as_ref()
  }
}
