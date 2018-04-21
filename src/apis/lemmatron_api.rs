/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct LemmatronApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> LemmatronApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> LemmatronApiClient<C> {
        LemmatronApiClient {
            configuration: configuration,
        }
    }
}

pub trait LemmatronApi {
    fn inflections_source_lang_word_id_filters_get(&self, source_lang: &str, word_id: &str, filters: Vec<String>, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Lemmatron, Error = Error<serde_json::Value>>>;
    fn inflections_source_lang_word_id_get(&self, source_lang: &str, word_id: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Lemmatron, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>LemmatronApi for LemmatronApiClient<C> {
    fn inflections_source_lang_word_id_filters_get(&self, source_lang: &str, word_id: &str, filters: Vec<String>, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Lemmatron, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/inflections/{source_lang}/{word_id}/{filters}?{}", configuration.base_path, query_string, source_lang=source_lang, word_id=word_id, filters=filters.join(","));

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("app_id", app_id);
            headers.set_raw("app_key", app_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::Lemmatron, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn inflections_source_lang_word_id_get(&self, source_lang: &str, word_id: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Lemmatron, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/inflections/{source_lang}/{word_id}?{}", configuration.base_path, query_string, source_lang=source_lang, word_id=word_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("app_id", app_id);
            headers.set_raw("app_key", app_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::Lemmatron, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}