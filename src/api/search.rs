//! Endpoints for search API

use crate::client::InvezgoClient;


pub struct SearchApi {
    pub(crate) client: InvezgoClient,
}

impl SearchApi {

    /// Cari Saham atau Pengguna
    pub async fn search(&self, query: &str) -> Result<(), crate::error::InvezgoError> {
        let path = "/search".to_string();
        let query = [
            ("query", Some(query.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Cari Saham
    pub async fn search_stock(&self, query: &str, cursor: &str) -> Result<(), crate::error::InvezgoError> {
        let path = "/search/stock".to_string();
        let query = [
            ("query", Some(query.to_string())),
            ("cursor", Some(cursor.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Cari Pengguna
    pub async fn search_user(&self, query: &str, cursor: &str) -> Result<(), crate::error::InvezgoError> {
        let path = "/search/user".to_string();
        let query = [
            ("query", Some(query.to_string())),
            ("cursor", Some(cursor.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }

}
