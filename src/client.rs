//! Client configuration and main instance

use std::sync::Arc;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use crate::error::InvezgoError;

#[derive(Clone)]
pub struct InvezgoClient {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    http: reqwest::Client,
    base_url: String,
}

impl InvezgoClient {
    pub fn builder() -> InvezgoClientBuilder {
        InvezgoClientBuilder::default()
    }

    pub(crate) async fn request<T, R>(&self, method: reqwest::Method, path: &str, query: &[(&str, Option<String>)], body: Option<&T>) -> Result<R, InvezgoError>
    where
        T: serde::Serialize + ?Sized,
        R: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.inner.base_url, path);
        let mut req = self.inner.http.request(method, &url);

        let mut query_params = Vec::new();
        for (k, v) in query {
            if let Some(val) = v {
                query_params.push((*k, val.clone()));
            }
        }
        if !query_params.is_empty() {
            req = req.query(&query_params);
        }

        if let Some(b) = body {
            req = req.json(b);
        }

        let res = req.send().await?;
        let status = res.status();

        if status.is_success() {
            if status == reqwest::StatusCode::NO_CONTENT {
                let body_text = res.text().await?;
                if body_text.is_empty() {
                    let val = serde_json::from_str::<R>("null")?;
                    return Ok(val);
                }
                let val = serde_json::from_str::<R>(&body_text)?;
                Ok(val)
            } else {
                let body_text = res.text().await?;
                let val = serde_json::from_str::<R>(&body_text)?;
                Ok(val)
            }
        } else {
            let body_text = res.text().await?;
            if let Ok(api_err) = serde_json::from_str::<crate::error::ApiErrorResponse>(&body_text) {
                Err(InvezgoError::ApiError {
                    status_code: status,
                    message: format!("{:?}", api_err.message),
                    error: api_err.error,
                })
            } else {
                Err(InvezgoError::ApiError {
                    status_code: status,
                    message: body_text,
                    error: None,
                })
            }
        }
    }

    pub fn alerts(&self) -> crate::api::alerts::AlertsApi {
        crate::api::alerts::AlertsApi { client: self.clone() }
    }
    pub fn analysis(&self) -> crate::api::analysis::AnalysisApi {
        crate::api::analysis::AnalysisApi { client: self.clone() }
    }
    pub fn journals(&self) -> crate::api::journals::JournalsApi {
        crate::api::journals::JournalsApi { client: self.clone() }
    }
    pub fn membership(&self) -> crate::api::membership::MembershipApi {
        crate::api::membership::MembershipApi { client: self.clone() }
    }
    pub fn portfolios(&self) -> crate::api::portfolios::PortfoliosApi {
        crate::api::portfolios::PortfoliosApi { client: self.clone() }
    }
    pub fn posts(&self) -> crate::api::posts::PostsApi {
        crate::api::posts::PostsApi { client: self.clone() }
    }
    pub fn recommendation(&self) -> crate::api::recommendation::RecommendationApi {
        crate::api::recommendation::RecommendationApi { client: self.clone() }
    }
    pub fn screener(&self) -> crate::api::screener::ScreenerApi {
        crate::api::screener::ScreenerApi { client: self.clone() }
    }
    pub fn search(&self) -> crate::api::search::SearchApi {
        crate::api::search::SearchApi { client: self.clone() }
    }
    pub fn trades(&self) -> crate::api::trades::TradesApi {
        crate::api::trades::TradesApi { client: self.clone() }
    }
    pub fn usage(&self) -> crate::api::usage::UsageApi {
        crate::api::usage::UsageApi { client: self.clone() }
    }
    pub fn watchlists(&self) -> crate::api::watchlists::WatchlistsApi {
        crate::api::watchlists::WatchlistsApi { client: self.clone() }
    }
}

#[derive(Default)]
pub struct InvezgoClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
}

impl InvezgoClientBuilder {
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<InvezgoClient, InvezgoError> {
        let api_key = self.api_key.ok_or_else(|| InvezgoError::Other("API key is required".to_string()))?;
        let base_url = self.base_url.unwrap_or_else(|| "https://api.invezgo.com".to_string());

        let mut headers = HeaderMap::new();
        let auth_val = format!("Bearer {}", api_key);
        let mut auth_header = HeaderValue::from_str(&auth_val)
            .map_err(|_| InvezgoError::Other("Invalid API key characters".to_string()))?;
        auth_header.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_header);

        let mut client_builder = reqwest::Client::builder()
            .default_headers(headers);

        if let Some(t) = self.timeout {
            client_builder = client_builder.timeout(t);
        }

        let http = client_builder.build()?;

        Ok(InvezgoClient {
            inner: Arc::new(ClientInner { http, base_url }),
        })
    }
}
