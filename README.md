# Rust API client for swagger

No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 1.11.0
- Package version: 1.0.0
- Build package: io.swagger.codegen.languages.RustClientCodegen

## Installation
Put the package under your project folder and add the following in import:
```
    "./swagger"
```

## Documentation for API Endpoints

All URIs are relative to *https://od-api-demo.oxforddictionaries.com:443/api/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DictionaryEntriesApi* | [**entries_source_lang_word_id_filters_get**](docs/DictionaryEntriesApi.md#entries_source_lang_word_id_filters_get) | **Get** /entries/{source_lang}/{word_id}/{filters} | Apply filters to response
*DictionaryEntriesApi* | [**entries_source_lang_word_id_get**](docs/DictionaryEntriesApi.md#entries_source_lang_word_id_get) | **Get** /entries/{source_lang}/{word_id} | Retrieve dictionary information for a given word
*DictionaryEntriesApi* | [**entries_source_lang_word_id_regionsregion_get**](docs/DictionaryEntriesApi.md#entries_source_lang_word_id_regionsregion_get) | **Get** /entries/{source_lang}/{word_id}/regions&#x3D;{region} | Specify GB or US dictionary for English entry search
*LemmatronApi* | [**inflections_source_lang_word_id_filters_get**](docs/LemmatronApi.md#inflections_source_lang_word_id_filters_get) | **Get** /inflections/{source_lang}/{word_id}/{filters} | Apply optional filters to Lemmatron response
*LemmatronApi* | [**inflections_source_lang_word_id_get**](docs/LemmatronApi.md#inflections_source_lang_word_id_get) | **Get** /inflections/{source_lang}/{word_id} | Check a word exists in the dictionary and retrieve its root form
*LexiStatsApi* | [**stats_frequency_ngrams_source_lang_corpus_ngram_size_get**](docs/LexiStatsApi.md#stats_frequency_ngrams_source_lang_corpus_ngram_size_get) | **Get** /stats/frequency/ngrams/{source_lang}/{corpus}/{ngram-size}/ | Retrieve the frequency of ngrams (1-4) derived from a corpus
*LexiStatsApi* | [**stats_frequency_word_source_lang_get**](docs/LexiStatsApi.md#stats_frequency_word_source_lang_get) | **Get** /stats/frequency/word/{source_lang}/ | Retrieve the frequency of a word derived from a corpus.
*LexiStatsApi* | [**stats_frequency_words_source_lang_get**](docs/LexiStatsApi.md#stats_frequency_words_source_lang_get) | **Get** /stats/frequency/words/{source_lang}/ | Retrieve a list of frequencies of a word/words derived from a corpus.
*SearchApi* | [**search_source_lang_get**](docs/SearchApi.md#search_source_lang_get) | **Get** /search/{source_lang} | Retrieve possible matches to input
*SearchApi* | [**search_source_search_language_translationstarget_search_language_get**](docs/SearchApi.md#search_source_search_language_translationstarget_search_language_get) | **Get** /search/{source_search_language}/translations&#x3D;{target_search_language} | Retrieve possible translation matches to input
*TheSentenceDictionaryApi* | [**entries_source_language_word_id_sentences_get**](docs/TheSentenceDictionaryApi.md#entries_source_language_word_id_sentences_get) | **Get** /entries/{source_language}/{word_id}/sentences | Retrieve corpus sentences for a given word
*ThesaurusApi* | [**entries_source_lang_word_id_antonyms_get**](docs/ThesaurusApi.md#entries_source_lang_word_id_antonyms_get) | **Get** /entries/{source_lang}/{word_id}/antonyms | Retrieve words that mean the opposite
*ThesaurusApi* | [**entries_source_lang_word_id_synonyms_get**](docs/ThesaurusApi.md#entries_source_lang_word_id_synonyms_get) | **Get** /entries/{source_lang}/{word_id}/synonyms | Retrieve words that are similar
*ThesaurusApi* | [**entries_source_lang_word_id_synonymsantonyms_get**](docs/ThesaurusApi.md#entries_source_lang_word_id_synonymsantonyms_get) | **Get** /entries/{source_lang}/{word_id}/synonyms;antonyms | Retrieve synonyms and antonyms for a given word
*TranslationApi* | [**entries_source_translation_language_word_id_translationstarget_translation_language_get**](docs/TranslationApi.md#entries_source_translation_language_word_id_translationstarget_translation_language_get) | **Get** /entries/{source_translation_language}/{word_id}/translations&#x3D;{target_translation_language} | Retrieve translation for a given word
*UtilityApi* | [**domains_source_domains_language_target_domains_language_get**](docs/UtilityApi.md#domains_source_domains_language_target_domains_language_get) | **Get** /domains/{source_domains_language}/{target_domains_language} | Lists available domains in a bilingual dataset
*UtilityApi* | [**domains_source_language_get**](docs/UtilityApi.md#domains_source_language_get) | **Get** /domains/{source_language} | Lists available domains in a monolingual dataset
*UtilityApi* | [**filters_endpoint_get**](docs/UtilityApi.md#filters_endpoint_get) | **Get** /filters/{endpoint} | Lists available filters for specific endpoint
*UtilityApi* | [**filters_get**](docs/UtilityApi.md#filters_get) | **Get** /filters | Lists available filters
*UtilityApi* | [**grammatical_features_source_language_get**](docs/UtilityApi.md#grammatical_features_source_language_get) | **Get** /grammaticalFeatures/{source_language} | Lists available grammatical features in a dataset
*UtilityApi* | [**languages_get**](docs/UtilityApi.md#languages_get) | **Get** /languages | Lists available dictionaries
*UtilityApi* | [**lexicalcategories_language_get**](docs/UtilityApi.md#lexicalcategories_language_get) | **Get** /lexicalcategories/{language} | Lists available lexical categories in a dataset
*UtilityApi* | [**regions_source_language_get**](docs/UtilityApi.md#regions_source_language_get) | **Get** /regions/{source_language} | Lists available regions in a monolingual dataset
*UtilityApi* | [**registers_source_language_get**](docs/UtilityApi.md#registers_source_language_get) | **Get** /registers/{source_language} | Lists available registers in a  monolingual dataset
*UtilityApi* | [**registers_source_register_language_target_register_language_get**](docs/UtilityApi.md#registers_source_register_language_target_register_language_get) | **Get** /registers/{source_register_language}/{target_register_language} | Lists available registers in a bilingual dataset
*WordlistApi* | [**wordlist_source_lang_filters_advanced_get**](docs/WordlistApi.md#wordlist_source_lang_filters_advanced_get) | **Get** /wordlist/{source_lang}/{filters_advanced} | Retrieve list of words for category with advanced options
*WordlistApi* | [**wordlist_source_lang_filters_basic_get**](docs/WordlistApi.md#wordlist_source_lang_filters_basic_get) | **Get** /wordlist/{source_lang}/{filters_basic} | Retrieve a list of words in a category


## Documentation For Models

 - [ArrayOfRelatedEntries](docs/ArrayOfRelatedEntries.md)
 - [ArrayOfRelatedEntriesInner](docs/ArrayOfRelatedEntriesInner.md)
 - [Arrayofstrings](docs/Arrayofstrings.md)
 - [CategorizedTextList](docs/CategorizedTextList.md)
 - [CategorizedTextListInner](docs/CategorizedTextListInner.md)
 - [CrossReferencesList](docs/CrossReferencesList.md)
 - [CrossReferencesListInner](docs/CrossReferencesListInner.md)
 - [Entry](docs/Entry.md)
 - [ExamplesList](docs/ExamplesList.md)
 - [ExamplesListInner](docs/ExamplesListInner.md)
 - [Filters](docs/Filters.md)
 - [FiltersResults](docs/FiltersResults.md)
 - [GrammaticalFeaturesList](docs/GrammaticalFeaturesList.md)
 - [GrammaticalFeaturesListInner](docs/GrammaticalFeaturesListInner.md)
 - [HeadwordEntry](docs/HeadwordEntry.md)
 - [HeadwordLemmatron](docs/HeadwordLemmatron.md)
 - [HeadwordThesaurus](docs/HeadwordThesaurus.md)
 - [InflectionsList](docs/InflectionsList.md)
 - [InflectionsListInner](docs/InflectionsListInner.md)
 - [Languages](docs/Languages.md)
 - [LanguagesResults](docs/LanguagesResults.md)
 - [LanguagesSourceLanguage](docs/LanguagesSourceLanguage.md)
 - [LanguagesTargetLanguage](docs/LanguagesTargetLanguage.md)
 - [Lemmatron](docs/Lemmatron.md)
 - [LemmatronLexicalEntry](docs/LemmatronLexicalEntry.md)
 - [LexicalEntry](docs/LexicalEntry.md)
 - [NgramsResult](docs/NgramsResult.md)
 - [NgramsResultResults](docs/NgramsResultResults.md)
 - [PronunciationsList](docs/PronunciationsList.md)
 - [PronunciationsListInner](docs/PronunciationsListInner.md)
 - [Regions](docs/Regions.md)
 - [RetrieveEntry](docs/RetrieveEntry.md)
 - [Sense](docs/Sense.md)
 - [SentencesEntry](docs/SentencesEntry.md)
 - [SentencesLexicalEntry](docs/SentencesLexicalEntry.md)
 - [SentencesResults](docs/SentencesResults.md)
 - [StatsWordResult](docs/StatsWordResult.md)
 - [StatsWordResultList](docs/StatsWordResultList.md)
 - [StatsWordResultListResults](docs/StatsWordResultListResults.md)
 - [StatsWordResultResult](docs/StatsWordResultResult.md)
 - [SynonymsAntonyms](docs/SynonymsAntonyms.md)
 - [SynonymsAntonymsInner](docs/SynonymsAntonymsInner.md)
 - [Thesaurus](docs/Thesaurus.md)
 - [ThesaurusEntry](docs/ThesaurusEntry.md)
 - [ThesaurusLexicalEntry](docs/ThesaurusLexicalEntry.md)
 - [ThesaurusLink](docs/ThesaurusLink.md)
 - [ThesaurusSense](docs/ThesaurusSense.md)
 - [TranslationsList](docs/TranslationsList.md)
 - [TranslationsListInner](docs/TranslationsListInner.md)
 - [UtilityLabels](docs/UtilityLabels.md)
 - [UtilityLabelsResults](docs/UtilityLabelsResults.md)
 - [VariantFormsList](docs/VariantFormsList.md)
 - [VariantFormsListInner](docs/VariantFormsListInner.md)
 - [Wordlist](docs/Wordlist.md)
 - [WordlistResults](docs/WordlistResults.md)


## Documentation For Authorization
 Endpoints do not require authorization.


## Author



