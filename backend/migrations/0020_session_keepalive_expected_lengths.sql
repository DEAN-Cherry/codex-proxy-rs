ALTER TABLE provider_accounts
    ADD COLUMN session_keepalive_expected_lengths INTEGER[] DEFAULT NULL
    CHECK (
        session_keepalive_expected_lengths IS NULL
        OR cardinality(session_keepalive_expected_lengths) BETWEEN 1 AND 32
    );

UPDATE provider_accounts
SET session_keepalive_expected_lengths = ARRAY[session_keepalive_expected_length]
WHERE session_keepalive_expected_length IS NOT NULL;
