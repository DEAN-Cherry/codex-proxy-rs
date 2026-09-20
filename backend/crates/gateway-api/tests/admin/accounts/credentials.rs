use gateway_api::admin::accounts::UpdateAccountRequest;
use serde_json::json;

#[test]
fn state_length_update_distinguishes_omission_null_and_value() {
    let mut body = json!({
        "accountId": "acct_state_length", "enabled": true,
        "concurrencyLimit": null, "weight": 1, "groupIds": []
    });
    let missing: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
    assert_eq!(missing.session_keepalive_expected_length, None);
    body["sessionKeepaliveExpectedLength"] = json!(null);
    let cleared: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
    assert_eq!(cleared.session_keepalive_expected_length, Some(None));
    for length in [100, 292, 332, 356, 2000] {
        body["sessionKeepaliveExpectedLength"] = json!(length);
        let request: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
        assert_eq!(
            request.session_keepalive_expected_length,
            Some(Some(length))
        );
        assert!(request.validate().is_ok());
    }
    for length in [0, 99, 2001, u32::MAX] {
        body["sessionKeepaliveExpectedLength"] = json!(length);
        let request: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
        assert!(request.validate().is_err());
    }
    for value in [json!(-1), json!(292.5), json!("332")] {
        body["sessionKeepaliveExpectedLength"] = value;
        assert!(serde_json::from_value::<UpdateAccountRequest>(body.clone()).is_err());
    }
}
