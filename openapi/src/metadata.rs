use std::collections::BTreeMap;

#[rustfmt::skip]
pub fn feature_groups() -> BTreeMap<&'static str, &'static str> {
    [
		// N.B. For now both `core` and `payment-methods` are always enabled.
		/*
		// Core Resources
		("balance", "core"),
		("balance_transaction", "core"),
		("charge", "core"),
		("customer", "core"),
		("dispute", "core"),
		("file", "core"),
		("file_link", "core"),
		("setup_intent", "core"),
		("payout", "core"),
		("platform_tax_fee", "core"),
		("product", "core"),
		("refund", "core"),
		("reserve_transaction", "core"),
		("token", "core"),

		// Payment Methods
        ("alipay_account", "payment-methods"),
		("bank_account", "payment-methods"),
		("payment_method", "payment-methods"),
		("source", "payment-methods"),
		*/

		// Checkout
		("checkout_session", "checkout"),

		// Billing (aka. Subscriptions)
		("coupon", "billing"),
		("discount", "billing"),
		("invoice", "billing"),
		("invoiceitem", "billing"),
        ("line_item", "billing"),
		("plan", "billing"),
		("subscription", "billing"),
		("subscription_item", "billing"),
		("subscription_schedule", "billing"),
 		("subscription_schedule_revision", "billing"),
        ("tax_id", "billing"),
		("tax_rate", "billing"),

		// Connect
		("account", "connect"),
		("application", "connect"),
		("application_fee", "connect"),
		("connect_collection_transfer", "connect"),
		("fee_refund", "connect"),
		("person", "connect"),
		("recipient", "connect"),
		("topup", "connect"),
		("transfer", "connect"),
		("transfer_reversal", "connect"),

		// Fraud
		("review", "fraud"),

		// Issuing
		("issuing.authorization", "issuing"),
		("issuing.card", "issuing"),
		("issuing.cardholder", "issuing"),
		("issuing.dispute", "issuing"),
		("issuing.transaction", "issuing"),

		// Orders
		("order", "orders"),
		("order_item", "orders"),
		("order_return", "orders"),
		("sku", "orders"),

		// Sigma
		("scheduled_query_run", "sigma"),

		// Webhooks Endpoints
		("webhook_endpoint", "webhook-endpoints"),
	]
	.iter()
	.copied()
	.collect()
}
