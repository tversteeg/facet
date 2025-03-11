use crate::from_urlencoded;
use shapely::Shapely;

#[derive(Debug, Shapely, PartialEq)]
struct SearchParams {
    query: String,
    page: u64,
}

#[test]
fn test_basic_urlencoded() {
    let query_string = "query=rust+programming&page=2";

    let mut partial = SearchParams::partial();
    from_urlencoded(&mut partial, query_string).expect("Failed to parse URL encoded data");

    let params = partial.build::<SearchParams>();
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

    let mut partial = SearchParams::partial();
    from_urlencoded(&mut partial, query_string).expect("Failed to parse URL encoded data");

    let params = partial.build::<SearchParams>();
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

    let mut partial = SearchParams::partial();
    from_urlencoded(&mut partial, query_string).expect("Failed to parse URL encoded data");

    // This should panic because the 'page' field is not initialized
    let _params = partial.build::<SearchParams>();
}

#[test]
fn test_unknown_field() {
    let query_string = "query=rust+programming&page=2&unknown=value";

    let mut partial = SearchParams::partial();
    from_urlencoded(&mut partial, query_string).expect("Failed to parse URL encoded data");

    let params = partial.build::<SearchParams>();
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

    let mut partial = SearchParams::partial();
    let result = from_urlencoded(&mut partial, query_string);

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
