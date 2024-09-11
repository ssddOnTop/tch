use std::num::NonZeroU64;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use async_graphql_value::ConstValue;
use cache_control::{Cachability, CacheControl};
use derive_setters::Setters;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use crate::blueprint::{Blueprint, Server, Upstream};
use crate::ir::IoId;

#[derive(Setters)]
pub struct RequestContext {
    pub server: Server,
    pub upstream: Upstream,
    pub x_response_headers: Arc<Mutex<HeaderMap>>,
    pub cookie_headers: Option<Arc<Mutex<HeaderMap>>>,
    // A subset of all the headers received in the GraphQL Request that will be sent to the
    // upstream.
    pub allowed_headers: HeaderMap,
    pub http_data_loaders: Arc<Vec<DataLoader<DataLoaderRequest, HttpDataLoader>>>,
    pub min_max_age: Arc<Mutex<Option<i32>>>,
    pub cache_public: Arc<Mutex<Option<bool>>>,
    pub runtime: TargetRuntime,
    pub cache: DedupeResult<IoId, ConstValue, anyhow::Error>,
    pub dedupe_handler: Arc<DedupeResult<IoId, ConstValue, anyhow::Error>>,
}

impl RequestContext {
    pub fn new(target_runtime: TargetRuntime) -> RequestContext {
        RequestContext {
            server: Default::default(),
            upstream: Default::default(),
            x_response_headers: Arc::new(Mutex::new(HeaderMap::new())),
            cookie_headers: None,
            http_data_loaders: Arc::new(vec![]),
            min_max_age: Arc::new(Mutex::new(None)),
            cache_public: Arc::new(Mutex::new(None)),
            runtime: target_runtime,
            cache: DedupeResult::new(true),
            dedupe_handler: Arc::new(DedupeResult::new(false)),
            allowed_headers: HeaderMap::new(),
        }
    }
    fn set_min_max_age_conc(&self, min_max_age: i32) {
        *self.min_max_age.lock().unwrap() = Some(min_max_age);
    }
    pub fn get_min_max_age(&self) -> Option<i32> {
        *self.min_max_age.lock().unwrap()
    }

    pub fn set_cache_public_false(&self) {
        *self.cache_public.lock().unwrap() = Some(false);
    }

    pub fn is_cache_public(&self) -> Option<bool> {
        *self.cache_public.lock().unwrap()
    }

    pub fn set_min_max_age(&self, max_age: i32) {
        let min_max_age_lock = self.get_min_max_age();
        match min_max_age_lock {
            Some(min_max_age) if max_age < min_max_age => {
                self.set_min_max_age_conc(max_age);
            }
            None => {
                self.set_min_max_age_conc(max_age);
            }
            _ => {}
        }
    }

    pub fn set_cache_visibility(&self, cachability: &Option<Cachability>) {
        if let Some(Cachability::Private) = cachability {
            self.set_cache_public_false()
        }
    }

    pub fn set_cache_control(&self, cache_policy: CacheControl) {
        if let Some(max_age) = cache_policy.max_age {
            self.set_min_max_age(max_age.as_secs() as i32);
        }
        self.set_cache_visibility(&cache_policy.cachability);
        if Some(Cachability::NoCache) == cache_policy.cachability {
            self.set_min_max_age(-1);
        }
    }

    pub fn set_cookie_headers(&self, headers: &HeaderMap) {
        // TODO fix execution_spec test and use append method
        // to allow multiple set cookie
        if let Some(map) = &self.cookie_headers {
            let map = &mut map.lock().unwrap();

            // Check if the incoming headers contain 'set-cookie'
            if let Some(new_cookies) = headers.get("set-cookie") {
                let cookie_name = HeaderName::from_str("set-cookie").unwrap();

                // Check if 'set-cookie' already exists in our map
                if let Some(existing_cookies) = map.get(&cookie_name) {
                    // Convert the existing HeaderValue to a str, append the new cookies,
                    // and then convert back to a HeaderValue. If the conversion fails, we skip
                    // appending.
                    if let Ok(existing_str) = existing_cookies.to_str() {
                        if let Ok(new_cookies_str) = new_cookies.to_str() {
                            // Create a new value by appending the new cookies to the existing ones
                            let combined_cookies = format!("{}; {}", existing_str, new_cookies_str);

                            // Replace the old value with the new, combined value
                            map.insert(
                                cookie_name,
                                HeaderValue::from_str(&combined_cookies).unwrap(),
                            );
                        }
                    }
                } else {
                    // If 'set-cookie' does not already exist in our map, just insert the new value
                    map.insert(cookie_name, new_cookies.clone());
                }
            }
        }
    }

    pub async fn cache_get(&self, key: &IoId) -> Result<Option<ConstValue>, anyhow::Error> {
        self.runtime.cache.get(key).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn cache_insert(
        &self,
        key: IoId,
        value: ConstValue,
        ttl: NonZeroU64,
    ) -> Result<(), anyhow::Error> {
        self.runtime.cache.set(key, value, ttl).await
    }
    

    /// Modifies existing headers to include the experimental headers
    pub fn extend_x_headers(&self, headers: &mut HeaderMap) {
        if self.has_experimental_headers() {
            let x_response_headers = &self.x_response_headers.lock().unwrap();
            for (header, value) in x_response_headers.iter() {
                headers.insert(header, value.clone());
            }
        }
    }
}

impl From<&Blueprint> for RequestContext {
    fn from(blueprint: &Blueprint) -> Self {
        Self {
            server: blueprint.server.clone(),
            upstream: blueprint.upstream.clone(),
            x_response_headers: Arc::new(Mutex::new(HeaderMap::new())),
            cookie_headers,
            allowed_headers: HeaderMap::new(),
            http_data_loaders: app_ctx.http_data_loaders.clone(),
                min_max_age: Arc::new(Mutex::new(None)),
            cache_public: Arc::new(Mutex::new(None)),
            runtime: app_ctx.runtime.clone(),
            cache: DedupeResult::new(true),
            dedupe_handler: app_ctx.dedupe_handler.clone(),
        }
    }
}
