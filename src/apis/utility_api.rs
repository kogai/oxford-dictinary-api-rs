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

pub struct UtilityApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> UtilityApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UtilityApiClient<C> {
        UtilityApiClient {
            configuration: configuration,
        }
    }
}

pub trait UtilityApi {
    fn domains_source_domains_language_target_domains_language_get(&self, source_domains_language: &str, target_domains_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>>;
    fn domains_source_language_get(&self, source_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>>;
    fn filters_endpoint_get(&self, endpoint: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Filters, Error = Error<serde_json::Value>>>;
    fn filters_get(&self, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Filters, Error = Error<serde_json::Value>>>;
    fn grammatical_features_source_language_get(&self, source_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>>;
    fn languages_get(&self, app_id: &str, app_key: &str, source_language: &str, target_language: &str) -> Box<Future<Item = ::models::Languages, Error = Error<serde_json::Value>>>;
    fn lexicalcategories_language_get(&self, language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>>;
    fn regions_source_language_get(&self, source_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Regions, Error = Error<serde_json::Value>>>;
    fn registers_source_language_get(&self, source_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>>;
    fn registers_source_register_language_target_register_language_get(&self, source_register_language: &str, target_register_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>UtilityApi for UtilityApiClient<C> {
    fn domains_source_domains_language_target_domains_language_get(&self, source_domains_language: &str, target_domains_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/domains/{source_domains_language}/{target_domains_language}?{}", configuration.base_path, query_string, source_domains_language=source_domains_language, target_domains_language=target_domains_language);

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
                let parsed: Result<::models::UtilityLabels, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn domains_source_language_get(&self, source_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/domains/{source_language}?{}", configuration.base_path, query_string, source_language=source_language);

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
                let parsed: Result<::models::UtilityLabels, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn filters_endpoint_get(&self, endpoint: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Filters, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/filters/{endpoint}?{}", configuration.base_path, query_string, endpoint=endpoint);

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
                let parsed: Result<::models::Filters, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn filters_get(&self, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Filters, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/filters?{}", configuration.base_path, query_string);

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
                let parsed: Result<::models::Filters, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn grammatical_features_source_language_get(&self, source_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/grammaticalFeatures/{source_language}?{}", configuration.base_path, query_string, source_language=source_language);

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
                let parsed: Result<::models::UtilityLabels, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn languages_get(&self, app_id: &str, app_key: &str, source_language: &str, target_language: &str) -> Box<Future<Item = ::models::Languages, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("sourceLanguage", &source_language.to_string());
            query.append_pair("targetLanguage", &target_language.to_string());
            query.finish()
        };
        let uri_str = format!("{}/languages?{}", configuration.base_path, query_string);

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
                let parsed: Result<::models::Languages, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn lexicalcategories_language_get(&self, language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/lexicalcategories/{language}?{}", configuration.base_path, query_string, language=language);

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
                let parsed: Result<::models::UtilityLabels, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn regions_source_language_get(&self, source_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::Regions, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/regions/{source_language}?{}", configuration.base_path, query_string, source_language=source_language);

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
                let parsed: Result<::models::Regions, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn registers_source_language_get(&self, source_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/registers/{source_language}?{}", configuration.base_path, query_string, source_language=source_language);

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
                let parsed: Result<::models::UtilityLabels, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn registers_source_register_language_target_register_language_get(&self, source_register_language: &str, target_register_language: &str, app_id: &str, app_key: &str) -> Box<Future<Item = ::models::UtilityLabels, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/registers/{source_register_language}/{target_register_language}?{}", configuration.base_path, query_string, source_register_language=source_register_language, target_register_language=target_register_language);

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
                let parsed: Result<::models::UtilityLabels, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}