use invezgo_sdk::{InvezgoClient, InvezgoError};
use invezgo_sdk::models::shared::CreateWatchlistDto;

#[test]
fn test_builder_requires_api_key() {
    let result = InvezgoClient::builder().build();
    assert!(result.is_err());
    if let Err(InvezgoError::Other(msg)) = result {
        assert_eq!(msg, "API key is required");
    } else {
        panic!("Expected InvezgoError::Other");
    }
}

#[test]
fn test_builder_success() {
    let client = InvezgoClient::builder()
        .api_key("my-secret-key")
        .base_url("https://api.invezgo.com")
        .build();
    assert!(client.is_ok());
}

#[test]
fn test_create_watchlist_serialization() {
    let dto = CreateWatchlistDto {
        group: "My Group".to_string(),
        code: "BBCA".to_string(),
        price: 9000.0,
        note: Some("Buy target".to_string()),
        scope: vec!["COMPOSITE".to_string()],
    };

    let serialized = serde_json::to_string(&dto).unwrap();
    assert!(serialized.contains(r#""group":"My Group""#));
    assert!(serialized.contains(r#""code":"BBCA""#));
    assert!(serialized.contains(r#""price":9000.0"#));
    assert!(serialized.contains(r#""note":"Buy target""#));
    assert!(serialized.contains(r#""scope":["COMPOSITE"]"#));
}
