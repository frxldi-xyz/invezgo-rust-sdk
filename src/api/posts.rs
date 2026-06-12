//! Endpoints for posts API

use crate::client::InvezgoClient;


pub struct PostsApi {
    pub(crate) client: InvezgoClient,
}

impl PostsApi {

    /// Daftar Semua Postingan
    pub async fn get_posts(&self, page: f64, limit: f64) -> Result<(), crate::error::InvezgoError> {
        let path = "/posts".to_string();
        let query = [
            ("page", Some(page.to_string())),
            ("limit", Some(limit.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Daftar Postingan Berdasarkan Kategori
    pub async fn get_category_posts(&self, category: &str, page: f64, limit: f64) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/posts/category/{}", category);
        let query = [
            ("page", Some(page.to_string())),
            ("limit", Some(limit.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Daftar Postingan Saham
    pub async fn get_stock_posts(&self, code: &str, page: f64, limit: f64) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/posts/space/{}", code);
        let query = [
            ("page", Some(page.to_string())),
            ("limit", Some(limit.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Daftar Postingan Saham Berdasarkan Kategori
    pub async fn get_stock_category_posts(&self, code: &str, category: &str, page: f64, limit: f64) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/posts/space/category/{}/{}", code, category);
        let query = [
            ("page", Some(page.to_string())),
            ("limit", Some(limit.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Isi Postingan
    pub async fn get_post_by_id(&self, id: &str) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/posts/detail/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Daftar Komentar Postingan
    pub async fn get_comment(&self, id: &str, page: f64, limit: f64) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/posts/comment/{}", id);
        let query = [
            ("page", Some(page.to_string())),
            ("limit", Some(limit.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Daftar Postingan Disukai
    pub async fn get_like(&self, page: f64, limit: f64) -> Result<(), crate::error::InvezgoError> {
        let path = "/posts/like".to_string();
        let query = [
            ("page", Some(page.to_string())),
            ("limit", Some(limit.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Daftar Postingan Favorit
    pub async fn get_favorite(&self, page: f64, limit: f64) -> Result<(), crate::error::InvezgoError> {
        let path = "/posts/favorite".to_string();
        let query = [
            ("page", Some(page.to_string())),
            ("limit", Some(limit.to_string())),
        ];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }


    /// Daftar Voting Postingan
    pub async fn get_voters(&self, id: &str) -> Result<(), crate::error::InvezgoError> {
        let path = format!("/posts/vote/{}", id);
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }

}
