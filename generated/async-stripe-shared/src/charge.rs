/// The `Charge` object represents a single attempt to move money into your Stripe account.
/// PaymentIntent confirmation is the most common way to create Charges, but transferring
/// money to a different Stripe account through Connect also creates Charges.
/// Some legacy payment flows create Charges directly, which is not recommended for new integrations.
///
/// For more details see <<https://stripe.com/docs/api/charges/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Charge {
    /// Amount intended to be collected by this payment.
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    /// Amount in cents (or local equivalent) captured (can be less than the amount attribute on the charge if a partial capture was made).
    pub amount_captured: i64,
    /// Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the charge if a partial refund was issued).
    pub amount_refunded: i64,
    /// ID of the Connect application that created the charge.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The application fee (if any) for the charge.
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collect-fees) for details.
    pub application_fee: Option<stripe_types::Expandable<stripe_shared::ApplicationFee>>,
    /// The amount of the application fee (if any) requested for the charge.
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collect-fees) for details.
    pub application_fee_amount: Option<i64>,
    /// Authorization code on the charge.
    pub authorization_code: Option<String>,
    /// ID of the balance transaction that describes the impact of this charge on your account balance (not including refunds or disputes).
    pub balance_transaction: Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    pub billing_details: stripe_shared::BillingDetails,
    /// The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card and bank statements.
    /// Allows you to see what the statement descriptor looks like after the static and dynamic portions are combined.
    /// This value only exists for card payments.
    pub calculated_statement_descriptor: Option<String>,
    /// If the charge was created without capturing, this Boolean represents whether it is still uncaptured or has since been captured.
    pub captured: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the customer this charge is for if one exists.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Whether the charge has been disputed.
    pub disputed: bool,
    /// ID of the balance transaction that describes the reversal of the balance on your account due to payment failure.
    pub failure_balance_transaction:
        Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>,
    /// Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/error-codes) for a list of codes).
    pub failure_code: Option<String>,
    /// Message to user further explaining reason for charge failure if available.
    pub failure_message: Option<String>,
    /// Information on fraud assessments for the charge.
    pub fraud_details: Option<stripe_shared::ChargeFraudDetails>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ChargeId,
    pub level3: Option<stripe_shared::Level3>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The account (if any) the charge was made on behalf of without triggering an automatic transfer.
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// Details about whether the payment was accepted, and why.
    /// See [understanding declines](https://stripe.com/docs/declines) for details.
    pub outcome: Option<stripe_shared::ChargeOutcome>,
    /// `true` if the charge succeeded, or was successfully authorized for later capture.
    pub paid: bool,
    /// ID of the PaymentIntent associated with this charge, if one exists.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    /// ID of the payment method used in this charge.
    pub payment_method: Option<String>,
    /// Details about the payment method at the time of the transaction.
    pub payment_method_details: Option<stripe_shared::PaymentMethodDetails>,
    pub presentment_details: Option<stripe_shared::PaymentFlowsPaymentIntentPresentmentDetails>,
    pub radar_options: Option<stripe_shared::RadarRadarOptions>,
    /// This is the email address that the receipt for this charge was sent to.
    pub receipt_email: Option<String>,
    /// This is the transaction number that appears on email receipts sent for this charge.
    /// This attribute will be `null` until a receipt has been sent.
    pub receipt_number: Option<String>,
    /// This is the URL to view the receipt for this charge.
    /// The receipt is kept up-to-date to the latest state of the charge, including any refunds.
    /// If the charge is for an Invoice, the receipt will be stylized as an Invoice receipt.
    pub receipt_url: Option<String>,
    /// Whether the charge has been fully refunded.
    /// If the charge is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    /// A list of refunds that have been applied to the charge.
    pub refunds: Option<stripe_types::List<stripe_shared::Refund>>,
    /// ID of the review associated with this charge if one exists.
    pub review: Option<stripe_types::Expandable<stripe_shared::Review>>,
    /// Shipping information for the charge.
    pub shipping: Option<stripe_shared::Shipping>,
    /// This is a legacy field that will be removed in the future.
    /// It contains the Source, Card, or BankAccount object used for the charge.
    /// For details about the payment method used for this charge, refer to `payment_method` or `payment_method_details` instead.
    pub source: Option<stripe_shared::PaymentSource>,
    /// The transfer ID which created this charge.
    /// Only present if the charge came from another Stripe account.
    /// [See the Connect documentation](https://docs.stripe.com/connect/destination-charges) for details.
    pub source_transfer: Option<stripe_types::Expandable<stripe_shared::Transfer>>,
    /// For a non-card charge, text that appears on the customer's statement as the statement descriptor.
    /// This value overrides the account's default statement descriptor.
    /// For information about requirements, including the 22-character limit, see [the Statement Descriptor docs](https://docs.stripe.com/get-started/account/statement-descriptors).
    ///
    /// For a card charge, this value is ignored unless you don't specify a `statement_descriptor_suffix`, in which case this value is used as the suffix.
    pub statement_descriptor: Option<String>,
    /// Provides information about a card charge.
    /// Concatenated to the account's [statement descriptor prefix](https://docs.stripe.com/get-started/account/statement-descriptors#static) to form the complete statement descriptor that appears on the customer's statement.
    /// If the account has no prefix value, the suffix is concatenated to the account's statement descriptor.
    pub statement_descriptor_suffix: Option<String>,
    /// The status of the payment is either `succeeded`, `pending`, or `failed`.
    pub status: ChargeStatus,
    /// ID of the transfer to the `destination` account (only applicable if the charge was created using the `destination` parameter).
    pub transfer: Option<stripe_types::Expandable<stripe_shared::Transfer>>,
    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub transfer_data: Option<stripe_shared::ChargeTransferData>,
    /// A string that identifies this transaction as part of a group.
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}
#[doc(hidden)]
pub struct ChargeBuilder {
    amount: Option<i64>,
    amount_captured: Option<i64>,
    amount_refunded: Option<i64>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    application_fee: Option<Option<stripe_types::Expandable<stripe_shared::ApplicationFee>>>,
    application_fee_amount: Option<Option<i64>>,
    authorization_code: Option<Option<String>>,
    balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    billing_details: Option<stripe_shared::BillingDetails>,
    calculated_statement_descriptor: Option<Option<String>>,
    captured: Option<bool>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    description: Option<Option<String>>,
    disputed: Option<bool>,
    failure_balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::BalanceTransaction>>>,
    failure_code: Option<Option<String>>,
    failure_message: Option<Option<String>>,
    fraud_details: Option<Option<stripe_shared::ChargeFraudDetails>>,
    id: Option<stripe_shared::ChargeId>,
    level3: Option<Option<stripe_shared::Level3>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    outcome: Option<Option<stripe_shared::ChargeOutcome>>,
    paid: Option<bool>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    payment_method: Option<Option<String>>,
    payment_method_details: Option<Option<stripe_shared::PaymentMethodDetails>>,
    presentment_details: Option<Option<stripe_shared::PaymentFlowsPaymentIntentPresentmentDetails>>,
    radar_options: Option<Option<stripe_shared::RadarRadarOptions>>,
    receipt_email: Option<Option<String>>,
    receipt_number: Option<Option<String>>,
    receipt_url: Option<Option<String>>,
    refunded: Option<bool>,
    refunds: Option<Option<stripe_types::List<stripe_shared::Refund>>>,
    review: Option<Option<stripe_types::Expandable<stripe_shared::Review>>>,
    shipping: Option<Option<stripe_shared::Shipping>>,
    source: Option<Option<stripe_shared::PaymentSource>>,
    source_transfer: Option<Option<stripe_types::Expandable<stripe_shared::Transfer>>>,
    statement_descriptor: Option<Option<String>>,
    statement_descriptor_suffix: Option<Option<String>>,
    status: Option<ChargeStatus>,
    transfer: Option<Option<stripe_types::Expandable<stripe_shared::Transfer>>>,
    transfer_data: Option<Option<stripe_shared::ChargeTransferData>>,
    transfer_group: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Charge {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Charge>,
        builder: ChargeBuilder,
    }

    impl Visitor for Place<Charge> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ChargeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ChargeBuilder {
        type Out = Charge;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_captured" => Deserialize::begin(&mut self.amount_captured),
                "amount_refunded" => Deserialize::begin(&mut self.amount_refunded),
                "application" => Deserialize::begin(&mut self.application),
                "application_fee" => Deserialize::begin(&mut self.application_fee),
                "application_fee_amount" => Deserialize::begin(&mut self.application_fee_amount),
                "authorization_code" => Deserialize::begin(&mut self.authorization_code),
                "balance_transaction" => Deserialize::begin(&mut self.balance_transaction),
                "billing_details" => Deserialize::begin(&mut self.billing_details),
                "calculated_statement_descriptor" => {
                    Deserialize::begin(&mut self.calculated_statement_descriptor)
                }
                "captured" => Deserialize::begin(&mut self.captured),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "description" => Deserialize::begin(&mut self.description),
                "disputed" => Deserialize::begin(&mut self.disputed),
                "failure_balance_transaction" => {
                    Deserialize::begin(&mut self.failure_balance_transaction)
                }
                "failure_code" => Deserialize::begin(&mut self.failure_code),
                "failure_message" => Deserialize::begin(&mut self.failure_message),
                "fraud_details" => Deserialize::begin(&mut self.fraud_details),
                "id" => Deserialize::begin(&mut self.id),
                "level3" => Deserialize::begin(&mut self.level3),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "outcome" => Deserialize::begin(&mut self.outcome),
                "paid" => Deserialize::begin(&mut self.paid),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "payment_method" => Deserialize::begin(&mut self.payment_method),
                "payment_method_details" => Deserialize::begin(&mut self.payment_method_details),
                "presentment_details" => Deserialize::begin(&mut self.presentment_details),
                "radar_options" => Deserialize::begin(&mut self.radar_options),
                "receipt_email" => Deserialize::begin(&mut self.receipt_email),
                "receipt_number" => Deserialize::begin(&mut self.receipt_number),
                "receipt_url" => Deserialize::begin(&mut self.receipt_url),
                "refunded" => Deserialize::begin(&mut self.refunded),
                "refunds" => Deserialize::begin(&mut self.refunds),
                "review" => Deserialize::begin(&mut self.review),
                "shipping" => Deserialize::begin(&mut self.shipping),
                "source" => Deserialize::begin(&mut self.source),
                "source_transfer" => Deserialize::begin(&mut self.source_transfer),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "statement_descriptor_suffix" => {
                    Deserialize::begin(&mut self.statement_descriptor_suffix)
                }
                "status" => Deserialize::begin(&mut self.status),
                "transfer" => Deserialize::begin(&mut self.transfer),
                "transfer_data" => Deserialize::begin(&mut self.transfer_data),
                "transfer_group" => Deserialize::begin(&mut self.transfer_group),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_captured: Deserialize::default(),
                amount_refunded: Deserialize::default(),
                application: Deserialize::default(),
                application_fee: Deserialize::default(),
                application_fee_amount: Deserialize::default(),
                authorization_code: Deserialize::default(),
                balance_transaction: Deserialize::default(),
                billing_details: Deserialize::default(),
                calculated_statement_descriptor: Deserialize::default(),
                captured: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                description: Deserialize::default(),
                disputed: Deserialize::default(),
                failure_balance_transaction: Deserialize::default(),
                failure_code: Deserialize::default(),
                failure_message: Deserialize::default(),
                fraud_details: Deserialize::default(),
                id: Deserialize::default(),
                level3: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                outcome: Deserialize::default(),
                paid: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_method: Deserialize::default(),
                payment_method_details: Deserialize::default(),
                presentment_details: Deserialize::default(),
                radar_options: Deserialize::default(),
                receipt_email: Deserialize::default(),
                receipt_number: Deserialize::default(),
                receipt_url: Deserialize::default(),
                refunded: Deserialize::default(),
                refunds: Deserialize::default(),
                review: Deserialize::default(),
                shipping: Deserialize::default(),
                source: Deserialize::default(),
                source_transfer: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                statement_descriptor_suffix: Deserialize::default(),
                status: Deserialize::default(),
                transfer: Deserialize::default(),
                transfer_data: Deserialize::default(),
                transfer_group: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_captured),
                Some(amount_refunded),
                Some(application),
                Some(application_fee),
                Some(application_fee_amount),
                Some(authorization_code),
                Some(balance_transaction),
                Some(billing_details),
                Some(calculated_statement_descriptor),
                Some(captured),
                Some(created),
                Some(currency),
                Some(customer),
                Some(description),
                Some(disputed),
                Some(failure_balance_transaction),
                Some(failure_code),
                Some(failure_message),
                Some(fraud_details),
                Some(id),
                Some(level3),
                Some(livemode),
                Some(metadata),
                Some(on_behalf_of),
                Some(outcome),
                Some(paid),
                Some(payment_intent),
                Some(payment_method),
                Some(payment_method_details),
                Some(presentment_details),
                Some(radar_options),
                Some(receipt_email),
                Some(receipt_number),
                Some(receipt_url),
                Some(refunded),
                Some(refunds),
                Some(review),
                Some(shipping),
                Some(source),
                Some(source_transfer),
                Some(statement_descriptor),
                Some(statement_descriptor_suffix),
                Some(status),
                Some(transfer),
                Some(transfer_data),
                Some(transfer_group),
            ) = (
                self.amount,
                self.amount_captured,
                self.amount_refunded,
                self.application.take(),
                self.application_fee.take(),
                self.application_fee_amount,
                self.authorization_code.take(),
                self.balance_transaction.take(),
                self.billing_details.take(),
                self.calculated_statement_descriptor.take(),
                self.captured,
                self.created,
                self.currency.take(),
                self.customer.take(),
                self.description.take(),
                self.disputed,
                self.failure_balance_transaction.take(),
                self.failure_code.take(),
                self.failure_message.take(),
                self.fraud_details.take(),
                self.id.take(),
                self.level3.take(),
                self.livemode,
                self.metadata.take(),
                self.on_behalf_of.take(),
                self.outcome.take(),
                self.paid,
                self.payment_intent.take(),
                self.payment_method.take(),
                self.payment_method_details.take(),
                self.presentment_details.take(),
                self.radar_options.take(),
                self.receipt_email.take(),
                self.receipt_number.take(),
                self.receipt_url.take(),
                self.refunded,
                self.refunds.take(),
                self.review.take(),
                self.shipping.take(),
                self.source.take(),
                self.source_transfer.take(),
                self.statement_descriptor.take(),
                self.statement_descriptor_suffix.take(),
                self.status,
                self.transfer.take(),
                self.transfer_data.take(),
                self.transfer_group.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_captured,
                amount_refunded,
                application,
                application_fee,
                application_fee_amount,
                authorization_code,
                balance_transaction,
                billing_details,
                calculated_statement_descriptor,
                captured,
                created,
                currency,
                customer,
                description,
                disputed,
                failure_balance_transaction,
                failure_code,
                failure_message,
                fraud_details,
                id,
                level3,
                livemode,
                metadata,
                on_behalf_of,
                outcome,
                paid,
                payment_intent,
                payment_method,
                payment_method_details,
                presentment_details,
                radar_options,
                receipt_email,
                receipt_number,
                receipt_url,
                refunded,
                refunds,
                review,
                shipping,
                source,
                source_transfer,
                statement_descriptor,
                statement_descriptor_suffix,
                status,
                transfer,
                transfer_data,
                transfer_group,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for Charge {
        type Builder = ChargeBuilder;
    }

    impl FromValueOpt for Charge {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ChargeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_captured" => b.amount_captured = FromValueOpt::from_value(v),
                    "amount_refunded" => b.amount_refunded = FromValueOpt::from_value(v),
                    "application" => b.application = FromValueOpt::from_value(v),
                    "application_fee" => b.application_fee = FromValueOpt::from_value(v),
                    "application_fee_amount" => {
                        b.application_fee_amount = FromValueOpt::from_value(v)
                    }
                    "authorization_code" => b.authorization_code = FromValueOpt::from_value(v),
                    "balance_transaction" => b.balance_transaction = FromValueOpt::from_value(v),
                    "billing_details" => b.billing_details = FromValueOpt::from_value(v),
                    "calculated_statement_descriptor" => {
                        b.calculated_statement_descriptor = FromValueOpt::from_value(v)
                    }
                    "captured" => b.captured = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "disputed" => b.disputed = FromValueOpt::from_value(v),
                    "failure_balance_transaction" => {
                        b.failure_balance_transaction = FromValueOpt::from_value(v)
                    }
                    "failure_code" => b.failure_code = FromValueOpt::from_value(v),
                    "failure_message" => b.failure_message = FromValueOpt::from_value(v),
                    "fraud_details" => b.fraud_details = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "level3" => b.level3 = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "on_behalf_of" => b.on_behalf_of = FromValueOpt::from_value(v),
                    "outcome" => b.outcome = FromValueOpt::from_value(v),
                    "paid" => b.paid = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "payment_method" => b.payment_method = FromValueOpt::from_value(v),
                    "payment_method_details" => {
                        b.payment_method_details = FromValueOpt::from_value(v)
                    }
                    "presentment_details" => b.presentment_details = FromValueOpt::from_value(v),
                    "radar_options" => b.radar_options = FromValueOpt::from_value(v),
                    "receipt_email" => b.receipt_email = FromValueOpt::from_value(v),
                    "receipt_number" => b.receipt_number = FromValueOpt::from_value(v),
                    "receipt_url" => b.receipt_url = FromValueOpt::from_value(v),
                    "refunded" => b.refunded = FromValueOpt::from_value(v),
                    "refunds" => b.refunds = FromValueOpt::from_value(v),
                    "review" => b.review = FromValueOpt::from_value(v),
                    "shipping" => b.shipping = FromValueOpt::from_value(v),
                    "source" => b.source = FromValueOpt::from_value(v),
                    "source_transfer" => b.source_transfer = FromValueOpt::from_value(v),
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),
                    "statement_descriptor_suffix" => {
                        b.statement_descriptor_suffix = FromValueOpt::from_value(v)
                    }
                    "status" => b.status = FromValueOpt::from_value(v),
                    "transfer" => b.transfer = FromValueOpt::from_value(v),
                    "transfer_data" => b.transfer_data = FromValueOpt::from_value(v),
                    "transfer_group" => b.transfer_group = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Charge {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Charge", 48)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_captured", &self.amount_captured)?;
        s.serialize_field("amount_refunded", &self.amount_refunded)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("application_fee", &self.application_fee)?;
        s.serialize_field("application_fee_amount", &self.application_fee_amount)?;
        s.serialize_field("authorization_code", &self.authorization_code)?;
        s.serialize_field("balance_transaction", &self.balance_transaction)?;
        s.serialize_field("billing_details", &self.billing_details)?;
        s.serialize_field(
            "calculated_statement_descriptor",
            &self.calculated_statement_descriptor,
        )?;
        s.serialize_field("captured", &self.captured)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("disputed", &self.disputed)?;
        s.serialize_field("failure_balance_transaction", &self.failure_balance_transaction)?;
        s.serialize_field("failure_code", &self.failure_code)?;
        s.serialize_field("failure_message", &self.failure_message)?;
        s.serialize_field("fraud_details", &self.fraud_details)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("level3", &self.level3)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("outcome", &self.outcome)?;
        s.serialize_field("paid", &self.paid)?;
        s.serialize_field("payment_intent", &self.payment_intent)?;
        s.serialize_field("payment_method", &self.payment_method)?;
        s.serialize_field("payment_method_details", &self.payment_method_details)?;
        s.serialize_field("presentment_details", &self.presentment_details)?;
        s.serialize_field("radar_options", &self.radar_options)?;
        s.serialize_field("receipt_email", &self.receipt_email)?;
        s.serialize_field("receipt_number", &self.receipt_number)?;
        s.serialize_field("receipt_url", &self.receipt_url)?;
        s.serialize_field("refunded", &self.refunded)?;
        s.serialize_field("refunds", &self.refunds)?;
        s.serialize_field("review", &self.review)?;
        s.serialize_field("shipping", &self.shipping)?;
        s.serialize_field("source", &self.source)?;
        s.serialize_field("source_transfer", &self.source_transfer)?;
        s.serialize_field("statement_descriptor", &self.statement_descriptor)?;
        s.serialize_field("statement_descriptor_suffix", &self.statement_descriptor_suffix)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("transfer", &self.transfer)?;
        s.serialize_field("transfer_data", &self.transfer_data)?;
        s.serialize_field("transfer_group", &self.transfer_group)?;

        s.serialize_field("object", "charge")?;
        s.end()
    }
}
/// The status of the payment is either `succeeded`, `pending`, or `failed`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ChargeStatus {
    Failed,
    Pending,
    Succeeded,
}
impl ChargeStatus {
    pub fn as_str(self) -> &'static str {
        use ChargeStatus::*;
        match self {
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for ChargeStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChargeStatus::*;
        match s {
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ChargeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ChargeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ChargeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ChargeStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ChargeStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ChargeStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ChargeStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ChargeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ChargeStatus"))
    }
}
impl stripe_types::Object for Charge {
    type Id = stripe_shared::ChargeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ChargeId);
