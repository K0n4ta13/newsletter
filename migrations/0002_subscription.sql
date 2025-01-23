create table subscription
(
    subscription_id uuid primary key                                 default uuid_generate_v1mc(),
    name            text collate "case_insensitive" unique not null,
    email           text collate "case_insensitive" unique not null,
    created_at      timestamptz                            not null  default now(),
    updated_at      timestamptz
);

SELECT trigger_updated_at('subscription');
