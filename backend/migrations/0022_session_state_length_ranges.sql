-- 使用紧凑 JSON 规则保留旧单值列表，同时支持闭区间，避免按跨度展开。
ALTER TABLE provider_accounts
    DROP CONSTRAINT provider_accounts_session_keepalive_expected_lengths_check,
    ALTER COLUMN session_keepalive_expected_lengths TYPE JSONB
        USING to_jsonb(session_keepalive_expected_lengths),
    ADD CONSTRAINT provider_accounts_session_keepalive_expected_lengths_check CHECK (
        session_keepalive_expected_lengths IS NULL
        OR (
            jsonb_typeof(session_keepalive_expected_lengths) = 'array'
            AND jsonb_array_length(session_keepalive_expected_lengths) BETWEEN 1 AND 32
        )
    );
