//! Tests for common functionality

use redis_client::common::*;
use redis_client::error::Result;

#[tokio::test]
async fn test_string_types_serialization() -> Result<()> {
    let set_request = SetStringRequest {
        value: "test_value".to_string(),
        ttl: Some(3600),
    };

    let json = serde_json::to_string(&set_request)?;
    let deserialized: SetStringRequest = serde_json::from_str(&json)?;

    assert_eq!(set_request.value, deserialized.value);
    assert_eq!(set_request.ttl, deserialized.ttl);

    Ok(())
}

#[tokio::test]
async fn test_set_types_serialization() -> Result<()> {
    let member_request = SetMemberRequest {
        member: "test_member".to_string(),
    };

    let json = serde_json::to_string(&member_request)?;
    let deserialized: SetMemberRequest = serde_json::from_str(&json)?;

    assert_eq!(member_request.member, deserialized.member);

    Ok(())
}

#[tokio::test]
async fn test_batch_get_request() -> Result<()> {
    let batch_request = BatchGetRequest {
        keys: vec!["key1".to_string(), "key2".to_string(), "key3".to_string()],
    };

    let json = serde_json::to_string(&batch_request)?;
    let deserialized: BatchGetRequest = serde_json::from_str(&json)?;

    assert_eq!(batch_request.keys, deserialized.keys);

    Ok(())
}

#[tokio::test]
async fn test_batch_set_request() -> Result<()> {
    let operations = vec![
        StringOperation {
            key: "key1".to_string(),
            value: Some("value1".to_string()),
            ttl: Some(3600),
        },
        StringOperation {
            key: "key2".to_string(),
            value: Some("value2".to_string()),
            ttl: None,
        }
    ];

    let batch_request = BatchSetRequest { operations };

    let json = serde_json::to_string(&batch_request)?;
    let deserialized: BatchSetRequest = serde_json::from_str(&json)?;

    assert_eq!(batch_request.operations.len(), deserialized.operations.len());

    Ok(())
}

#[tokio::test]
async fn test_string_info_serialization() -> Result<()> {
    let string_info = StringInfo {
        key: "test_key".to_string(),
        value: "test_value".to_string(),
        ttl: Some(3600),
        type_: "string".to_string(),
        encoding: "raw".to_string(),
        size: 10,
    };

    let json = serde_json::to_string(&string_info)?;
    let deserialized: StringInfo = serde_json::from_str(&json)?;

    assert_eq!(string_info.key, deserialized.key);
    assert_eq!(string_info.value, deserialized.value);
    assert_eq!(string_info.ttl, deserialized.ttl);
    assert_eq!(string_info.type_, deserialized.type_);
    assert_eq!(string_info.encoding, deserialized.encoding);
    assert_eq!(string_info.size, deserialized.size);

    Ok(())
}

#[tokio::test]
async fn test_set_info_serialization() -> Result<()> {
    let set_info = SetInfo {
        key: "test_set".to_string(),
        members: vec!["member1".to_string(), "member2".to_string()],
        cardinality: 2,
        ttl: Some(3600),
    };

    let json = serde_json::to_string(&set_info)?;
    let deserialized: SetInfo = serde_json::from_str(&json)?;

    assert_eq!(set_info.key, deserialized.key);
    assert_eq!(set_info.members, deserialized.members);
    assert_eq!(set_info.cardinality, deserialized.cardinality);
    assert_eq!(set_info.ttl, deserialized.ttl);

    Ok(())
}

#[tokio::test]
async fn test_api_response_serialization() -> Result<()> {
    let api_response = ApiResponse {
        success: true,
        data: Some("test_data".to_string()),
        error: None,
    };

    let json = serde_json::to_string(&api_response)?;
    let deserialized: ApiResponse<String> = serde_json::from_str(&json)?;

    assert_eq!(api_response.success, deserialized.success);
    assert_eq!(api_response.data, deserialized.data);
    assert_eq!(api_response.error, deserialized.error);

    Ok(())
}

#[tokio::test]
async fn test_pattern_results_serialization() -> Result<()> {
    let pattern_results = PatternResults {
        grouped: true,
        results: serde_json::json!({"pattern1": {"key1": "value1"}}),
    };

    let json = serde_json::to_string(&pattern_results)?;
    let deserialized: PatternResults = serde_json::from_str(&json)?;

    assert_eq!(pattern_results.grouped, deserialized.grouped);
    assert_eq!(pattern_results.results, deserialized.results);

    Ok(())
}
