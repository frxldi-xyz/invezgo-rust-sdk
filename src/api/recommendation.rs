//! Endpoints for recommendation API

use crate::client::InvezgoClient;


pub struct RecommendationApi {
    pub(crate) client: InvezgoClient,
}

impl RecommendationApi {

    /// Daftar Rekomendasi Pengguna
    pub async fn user_recommendations(&self) -> Result<(), crate::error::InvezgoError> {
        let path = "/recommendation/user".to_string();
        let query: &[(&str, Option<String>)] = &[];
        self.client.request(reqwest::Method::GET, &path, &query, None::<&()>).await
    }

}
