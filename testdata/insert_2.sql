/*
@name InsertShareClasses
@param rows -> ((portfolioId, shareClass, isin, currency, asOfDate, salesStatus, hexCode, complexity, subscriptionFee, redemptionFee, managementFee, runningCosts, terNumber, currencyExchangeCosts, performanceRenumeration, fundTransactionCosts, instrumentManagementCosts, rulesFi, rulesSv, brochureFi, brochureSv)...)
*/
INSERT INTO share_classes (
  portfolio_id,
  share_class,
  isin,
  currency,
  as_of_date,
  sales_status,
  hex_code,
  complexity,
  subscription_fee,
  redemption_fee,
  management_fee,
  running_costs,
  ter_number,
  currency_exchange_costs,
  performance_renumeration,
  fund_transaction_costs,
  instrument_management_costs,
  rules_fi,
  rules_sv,
  brochure_fi,
  brochure_sv
) VALUES :rows
ON CONFLICT (portfolio_id, share_class) DO UPDATE SET
  isin = EXCLUDED.isin,
  currency = EXCLUDED.currency,
  as_of_date = EXCLUDED.as_of_date,
  sales_status = EXCLUDED.sales_status,
  hex_code = EXCLUDED.hex_code,
  complexity = EXCLUDED.complexity,
  subscription_fee = EXCLUDED.subscription_fee,
  redemption_fee = EXCLUDED.redemption_fee,
  management_fee = EXCLUDED.management_fee,
  running_costs = EXCLUDED.running_costs,
  ter_number = EXCLUDED.ter_number,
  currency_exchange_costs = EXCLUDED.currency_exchange_costs,
  performance_renumeration = EXCLUDED.performance_renumeration,
  fund_transaction_costs = EXCLUDED.fund_transaction_costs,
  instrument_management_costs = EXCLUDED.instrument_management_costs,
  rules_fi = EXCLUDED.rules_fi,
  rules_sv = EXCLUDED.rules_sv,
  brochure_fi = EXCLUDED.brochure_fi,
  brochure_sv = EXCLUDED.brochure_sv;
