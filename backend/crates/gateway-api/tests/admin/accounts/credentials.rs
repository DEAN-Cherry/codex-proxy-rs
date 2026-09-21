use gateway_api::admin::accounts::UpdateAccountRequest;
use serde_json::json;

#[test]
fn state_length_update_distinguishes_omission_null_and_value() {
    let mut body = json!({
        "accountId": "acct_state_length", "enabled": true,
        "concurrencyLimit": null, "weight": 1, "groupIds": []
    });
    let missing: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
    assert_eq!(missing.session_keepalive_expected_lengths, None);
    body["sessionKeepaliveExpectedLengths"] = json!(null);
    let cleared: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
    assert_eq!(cleared.session_keepalive_expected_lengths, Some(None));
    for lengths in [vec![100], vec![292, 312], vec![332, 356], vec![2000]] {
        body["sessionKeepaliveExpectedLengths"] = json!(lengths);
        let request: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
        assert_eq!(
            request.session_keepalive_expected_lengths,
            Some(Some(lengths))
        );
        assert!(request.validate().is_ok());
    }
    for lengths in [
        vec![],
        vec![0],
        vec![99],
        vec![2001],
        vec![292, 292],
        vec![292; 33],
    ] {
        body["sessionKeepaliveExpectedLengths"] = json!(lengths);
        let request: UpdateAccountRequest = serde_json::from_value(body.clone()).unwrap();
        assert!(request.validate().is_err());
    }
    for value in [json!([-1]), json!([292.5]), json!(["332"])] {
        body["sessionKeepaliveExpectedLengths"] = value;
        assert!(serde_json::from_value::<UpdateAccountRequest>(body.clone()).is_err());
    }
}
