CREATE TABLE share_classes(
    portfolio_id INTEGER NOT NULL,
    share_class VARCHAR NOT NULL,
    isin VARCHAR NOT NULL,
    currency VARCHAR NOT NULL,
    as_of_date DATE NOT NULL,
    sales_status VARCHAR NOT NULL,
    hex_code VARCHAR NOT NULL,
    complexity VARCHAR NOT NULL,
    subscription_fee VARCHAR NOT NULL,
    redemption_fee VARCHAR NOT NULL,
    management_fee VARCHAR NOT NULL,
    running_costs VARCHAR NOT NULL,
    ter_number VARCHAR NOT NULL,
    currency_exchange_costs VARCHAR NOT NULL,
    performance_renumeration VARCHAR NOT NULL,
    fund_transaction_costs VARCHAR NOT NULL,
    instrument_management_costs VARCHAR NOT NULL,
    rules_fi VARCHAR NOT NULL,
    rules_sv VARCHAR NOT NULL,
    brochure_fi VARCHAR NOT NULL,
    brochure_sv VARCHAR NOT NULL
);

ALTER TABLE share_classes ADD CONSTRAINT share_classes_pk PRIMARY KEY (portfolio_id, share_class);

