/* 
 * Gaia-Lite for Cosmos
 *
 * A REST interface for state queries, transaction generation and broadcasting.
 *
 * OpenAPI spec version: 3.0
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

pub struct ICS21ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ICS21ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ICS21ApiClient<C> {
        ICS21ApiClient {
            configuration: configuration,
        }
    }
}

pub trait ICS21Api {
    fn staking_delegators_delegator_addr_delegations_get(&self, delegator_addr: &str) -> Box<Future<Item = Vec<::models::Delegation>, Error = Error<serde_json::Value>>>;
    fn staking_delegators_delegator_addr_delegations_post(&self, delegator_addr: &str, delegation: ::models::Delegation) -> Box<Future<Item = ::models::BroadcastTxCommitResult, Error = Error<serde_json::Value>>>;
    fn staking_delegators_delegator_addr_delegations_validator_addr_get(&self, delegator_addr: &str, validator_addr: &str) -> Box<Future<Item = ::models::Delegation, Error = Error<serde_json::Value>>>;
    fn staking_delegators_delegator_addr_redelegations_post(&self, delegator_addr: &str, delegation: ::models::Delegation2) -> Box<Future<Item = ::models::StdTx, Error = Error<serde_json::Value>>>;
    fn staking_delegators_delegator_addr_txs_get(&self, delegator_addr: &str) -> Box<Future<Item = Vec<::models::TxQuery>, Error = Error<serde_json::Value>>>;
    fn staking_delegators_delegator_addr_unbonding_delegations_get(&self, delegator_addr: &str) -> Box<Future<Item = Vec<::models::UnbondingDelegation>, Error = Error<serde_json::Value>>>;
    fn staking_delegators_delegator_addr_unbonding_delegations_post(&self, delegator_addr: &str, delegation: ::models::Delegation1) -> Box<Future<Item = ::models::BroadcastTxCommitResult, Error = Error<serde_json::Value>>>;
    fn staking_delegators_delegator_addr_unbonding_delegations_validator_addr_get(&self, delegator_addr: &str, validator_addr: &str) -> Box<Future<Item = Vec<::models::UnbondingDelegation>, Error = Error<serde_json::Value>>>;
    fn staking_delegators_delegator_addr_validators_get(&self, delegator_addr: &str) -> Box<Future<Item = Vec<::models::Validator>, Error = Error<serde_json::Value>>>;
    fn staking_delegators_delegator_addr_validators_validator_addr_get(&self, delegator_addr: &str, validator_addr: &str) -> Box<Future<Item = ::models::Validator, Error = Error<serde_json::Value>>>;
    fn staking_parameters_get(&self, ) -> Box<Future<Item = ::models::InlineResponse2005, Error = Error<serde_json::Value>>>;
    fn staking_pool_get(&self, ) -> Box<Future<Item = ::models::InlineResponse2004, Error = Error<serde_json::Value>>>;
    fn staking_redelegations_get(&self, delegator: &str, validator_from: &str, validator_to: &str) -> Box<Future<Item = Vec<::models::Redelegation>, Error = Error<serde_json::Value>>>;
    fn staking_validators_get(&self, status: &str, page: i32, limit: i32) -> Box<Future<Item = Vec<::models::Validator>, Error = Error<serde_json::Value>>>;
    fn staking_validators_validator_addr_delegations_get(&self, validator_addr: &str) -> Box<Future<Item = Vec<::models::Delegation>, Error = Error<serde_json::Value>>>;
    fn staking_validators_validator_addr_get(&self, validator_addr: &str) -> Box<Future<Item = ::models::Validator, Error = Error<serde_json::Value>>>;
    fn staking_validators_validator_addr_unbonding_delegations_get(&self, validator_addr: &str) -> Box<Future<Item = Vec<::models::UnbondingDelegation>, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>ICS21Api for ICS21ApiClient<C> {
    fn staking_delegators_delegator_addr_delegations_get(&self, delegator_addr: &str) -> Box<Future<Item = Vec<::models::Delegation>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/delegations?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<Vec<::models::Delegation>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_delegators_delegator_addr_delegations_post(&self, delegator_addr: &str, delegation: ::models::Delegation) -> Box<Future<Item = ::models::BroadcastTxCommitResult, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/delegations?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }



        let serialized = serde_json::to_string(&delegation).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

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
                let parsed: Result<::models::BroadcastTxCommitResult, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_delegators_delegator_addr_delegations_validator_addr_get(&self, delegator_addr: &str, validator_addr: &str) -> Box<Future<Item = ::models::Delegation, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/delegations/{validatorAddr}?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr, validatorAddr=validator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<::models::Delegation, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_delegators_delegator_addr_redelegations_post(&self, delegator_addr: &str, delegation: ::models::Delegation2) -> Box<Future<Item = ::models::StdTx, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/redelegations?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }



        let serialized = serde_json::to_string(&delegation).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

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
                let parsed: Result<::models::StdTx, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_delegators_delegator_addr_txs_get(&self, delegator_addr: &str) -> Box<Future<Item = Vec<::models::TxQuery>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/txs?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<Vec<::models::TxQuery>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_delegators_delegator_addr_unbonding_delegations_get(&self, delegator_addr: &str) -> Box<Future<Item = Vec<::models::UnbondingDelegation>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/unbonding_delegations?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<Vec<::models::UnbondingDelegation>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_delegators_delegator_addr_unbonding_delegations_post(&self, delegator_addr: &str, delegation: ::models::Delegation1) -> Box<Future<Item = ::models::BroadcastTxCommitResult, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/unbonding_delegations?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }



        let serialized = serde_json::to_string(&delegation).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

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
                let parsed: Result<::models::BroadcastTxCommitResult, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_delegators_delegator_addr_unbonding_delegations_validator_addr_get(&self, delegator_addr: &str, validator_addr: &str) -> Box<Future<Item = Vec<::models::UnbondingDelegation>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/unbonding_delegations/{validatorAddr}?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr, validatorAddr=validator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<Vec<::models::UnbondingDelegation>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_delegators_delegator_addr_validators_get(&self, delegator_addr: &str) -> Box<Future<Item = Vec<::models::Validator>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/validators?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<Vec<::models::Validator>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_delegators_delegator_addr_validators_validator_addr_get(&self, delegator_addr: &str, validator_addr: &str) -> Box<Future<Item = ::models::Validator, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/delegators/{delegatorAddr}/validators/{validatorAddr}?{}", configuration.base_path, query_string, delegatorAddr=delegator_addr, validatorAddr=validator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<::models::Validator, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_parameters_get(&self, ) -> Box<Future<Item = ::models::InlineResponse2005, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/parameters?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<::models::InlineResponse2005, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_pool_get(&self, ) -> Box<Future<Item = ::models::InlineResponse2004, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/pool?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<::models::InlineResponse2004, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_redelegations_get(&self, delegator: &str, validator_from: &str, validator_to: &str) -> Box<Future<Item = Vec<::models::Redelegation>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("delegator", &delegator.to_string());
            query.append_pair("validator_from", &validator_from.to_string());
            query.append_pair("validator_to", &validator_to.to_string());
            query.finish()
        };
        let uri_str = format!("{}/staking/redelegations?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<Vec<::models::Redelegation>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_validators_get(&self, status: &str, page: i32, limit: i32) -> Box<Future<Item = Vec<::models::Validator>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("status", &status.to_string());
            query.append_pair("page", &page.to_string());
            query.append_pair("limit", &limit.to_string());
            query.finish()
        };
        let uri_str = format!("{}/staking/validators?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<Vec<::models::Validator>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_validators_validator_addr_delegations_get(&self, validator_addr: &str) -> Box<Future<Item = Vec<::models::Delegation>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/validators/{validatorAddr}/delegations?{}", configuration.base_path, query_string, validatorAddr=validator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<Vec<::models::Delegation>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_validators_validator_addr_get(&self, validator_addr: &str) -> Box<Future<Item = ::models::Validator, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/validators/{validatorAddr}?{}", configuration.base_path, query_string, validatorAddr=validator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<::models::Validator, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn staking_validators_validator_addr_unbonding_delegations_get(&self, validator_addr: &str) -> Box<Future<Item = Vec<::models::UnbondingDelegation>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/staking/validators/{validatorAddr}/unbonding_delegations?{}", configuration.base_path, query_string, validatorAddr=validator_addr);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
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
                let parsed: Result<Vec<::models::UnbondingDelegation>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
