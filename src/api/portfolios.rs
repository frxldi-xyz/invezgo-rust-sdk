//! Endpoints for portfolios API

use crate::client::InvezgoClient;


pub struct PortfoliosApi {
    pub(crate) client: InvezgoClient,
}

impl PortfoliosApi {

    /// Daftar Portofolio
    pub async fn list_portfolio(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/portfolios".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Ringkasan Portofolio
    pub async fn portfolio_summary(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/portfolios/summary".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }

}
