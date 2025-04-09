use crate::from_str;
use facet_derive::Facet;
use facet_trait::{self as facet, Facet};

#[derive(Debug, Facet, PartialEq)]
struct SearchParams {
    query: String,
    page: u64,
}

#[test]
fn test_basic_urlencoded() {
    let query_string = "query=rust+programming&page=2";

    let params: SearchParams = from_str(query_string).expect("Failed to parse URL encoded data");
    assert_eq!(
        params,
        SearchParams {
            query: "rust programming".to_string(),
            page: 2
        }
    );
}

#[test]
fn test_encoded_characters() {
    let query_string = "query=rust%20programming%21&page=3";

    let params: SearchParams = from_str(query_string).expect("Failed to parse URL encoded data");
    assert_eq!(
        params,
        SearchParams {
            query: "rust programming!".to_string(),
            page: 3
        }
    );
}

#[test]
#[should_panic(expected = "Field 'page' was not initialized")]
fn test_missing_field() {
    let query_string = "query=rust+programming";

    // This should panic because the 'page' field is not initialized
    let _params: SearchParams = from_str(query_string).expect("Failed to parse URL encoded data");
}

#[test]
fn test_unknown_field() {
    let query_string = "query=rust+programming&page=2&unknown=value";

    let params: SearchParams = from_str(query_string).expect("Failed to parse URL encoded data");
    assert_eq!(
        params,
        SearchParams {
            query: "rust programming".to_string(),
            page: 2
        }
    );
}

#[test]
fn test_invalid_number() {
    let query_string = "query=rust+programming&page=not_a_number";

    let result = from_str::<SearchParams>(query_string);

    assert!(result.is_err());
    if let Err(err) = result {
        match err {
            crate::UrlEncodedError::InvalidNumber(field, value) => {
                assert_eq!(field, "page");
                assert_eq!(value, "not_a_number");
            }
            _ => panic!("Expected InvalidNumber error"),
        }
    }
}
