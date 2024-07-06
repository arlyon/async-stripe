#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum AccountExternalAccountCreated {
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "bank_account")
    )]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct AccountExternalAccountCreatedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<AccountExternalAccountCreated>,
        builder: AccountExternalAccountCreatedBuilder,
    }

    impl Deserialize for AccountExternalAccountCreated {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<AccountExternalAccountCreated> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for AccountExternalAccountCreatedBuilder {
        type Out = AccountExternalAccountCreated;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            AccountExternalAccountCreated::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for AccountExternalAccountCreated {
        type Builder = AccountExternalAccountCreatedBuilder;
    }
    impl AccountExternalAccountCreated {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for AccountExternalAccountCreated {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum AccountExternalAccountDeleted {
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "bank_account")
    )]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct AccountExternalAccountDeletedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<AccountExternalAccountDeleted>,
        builder: AccountExternalAccountDeletedBuilder,
    }

    impl Deserialize for AccountExternalAccountDeleted {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<AccountExternalAccountDeleted> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for AccountExternalAccountDeletedBuilder {
        type Out = AccountExternalAccountDeleted;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            AccountExternalAccountDeleted::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for AccountExternalAccountDeleted {
        type Builder = AccountExternalAccountDeletedBuilder;
    }
    impl AccountExternalAccountDeleted {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for AccountExternalAccountDeleted {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum AccountExternalAccountUpdated {
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "bank_account")
    )]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct AccountExternalAccountUpdatedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<AccountExternalAccountUpdated>,
        builder: AccountExternalAccountUpdatedBuilder,
    }

    impl Deserialize for AccountExternalAccountUpdated {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<AccountExternalAccountUpdated> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for AccountExternalAccountUpdatedBuilder {
        type Out = AccountExternalAccountUpdated;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            AccountExternalAccountUpdated::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for AccountExternalAccountUpdated {
        type Builder = AccountExternalAccountUpdatedBuilder;
    }
    impl AccountExternalAccountUpdated {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for AccountExternalAccountUpdated {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum CustomerSourceCreated {
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "bank_account")
    )]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct CustomerSourceCreatedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<CustomerSourceCreated>,
        builder: CustomerSourceCreatedBuilder,
    }

    impl Deserialize for CustomerSourceCreated {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<CustomerSourceCreated> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for CustomerSourceCreatedBuilder {
        type Out = CustomerSourceCreated;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            CustomerSourceCreated::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for CustomerSourceCreated {
        type Builder = CustomerSourceCreatedBuilder;
    }
    impl CustomerSourceCreated {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for CustomerSourceCreated {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum CustomerSourceDeleted {
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "bank_account")
    )]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct CustomerSourceDeletedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<CustomerSourceDeleted>,
        builder: CustomerSourceDeletedBuilder,
    }

    impl Deserialize for CustomerSourceDeleted {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<CustomerSourceDeleted> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for CustomerSourceDeletedBuilder {
        type Out = CustomerSourceDeleted;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            CustomerSourceDeleted::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for CustomerSourceDeleted {
        type Builder = CustomerSourceDeletedBuilder;
    }
    impl CustomerSourceDeleted {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for CustomerSourceDeleted {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum CustomerSourceExpiring {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct CustomerSourceExpiringBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<CustomerSourceExpiring>,
        builder: CustomerSourceExpiringBuilder,
    }

    impl Deserialize for CustomerSourceExpiring {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<CustomerSourceExpiring> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for CustomerSourceExpiringBuilder {
        type Out = CustomerSourceExpiring;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            CustomerSourceExpiring::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for CustomerSourceExpiring {
        type Builder = CustomerSourceExpiringBuilder;
    }
    impl CustomerSourceExpiring {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for CustomerSourceExpiring {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum CustomerSourceUpdated {
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "bank_account")
    )]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct CustomerSourceUpdatedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<CustomerSourceUpdated>,
        builder: CustomerSourceUpdatedBuilder,
    }

    impl Deserialize for CustomerSourceUpdated {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<CustomerSourceUpdated> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for CustomerSourceUpdatedBuilder {
        type Out = CustomerSourceUpdated;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            CustomerSourceUpdated::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for CustomerSourceUpdated {
        type Builder = CustomerSourceUpdatedBuilder;
    }
    impl CustomerSourceUpdated {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for CustomerSourceUpdated {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};

#[derive(Clone, Debug)]
#[non_exhaustive]
/// The event data for a webhook event.
pub enum EventObject {
    /// Occurs whenever a user authorizes an application. Sent to the related application only.
    AccountApplicationAuthorized(stripe_shared::Application),
    /// Occurs whenever a user deauthorizes an application. Sent to the related application only.
    AccountApplicationDeauthorized(stripe_shared::Application),
    /// Occurs whenever an external account is created.
    AccountExternalAccountCreated(AccountExternalAccountCreated),
    /// Occurs whenever an external account is deleted.
    AccountExternalAccountDeleted(AccountExternalAccountDeleted),
    /// Occurs whenever an external account is updated.
    AccountExternalAccountUpdated(AccountExternalAccountUpdated),
    /// Occurs whenever an account status or property has changed.
    AccountUpdated(stripe_shared::Account),
    /// Occurs whenever an application fee is created on a charge.
    ApplicationFeeCreated(stripe_shared::ApplicationFee),
    /// Occurs whenever an application fee refund is updated.
    ApplicationFeeRefundUpdated(stripe_shared::ApplicationFeeRefund),
    /// Occurs whenever an application fee is refunded, whether from refunding a charge or from [refunding the application fee directly](#fee_refunds).
    /// This includes partial refunds.
    ApplicationFeeRefunded(stripe_shared::ApplicationFee),
    /// Occurs whenever your Stripe balance has been updated (e.g., when a charge is available to be paid out).
    /// By default, Stripe automatically transfers funds in your balance to your bank account on a daily basis.
    /// This event is not fired for negative transactions.
    #[cfg(feature = "stripe_core")]
    BalanceAvailable(stripe_core::Balance),
    /// Occurs whenever a portal configuration is created.
    #[cfg(feature = "stripe_billing")]
    BillingPortalConfigurationCreated(stripe_billing::BillingPortalConfiguration),
    /// Occurs whenever a portal configuration is updated.
    #[cfg(feature = "stripe_billing")]
    BillingPortalConfigurationUpdated(stripe_billing::BillingPortalConfiguration),
    /// Occurs whenever a portal session is created.
    #[cfg(feature = "stripe_billing")]
    BillingPortalSessionCreated(stripe_billing::BillingPortalSession),
    /// Occurs whenever a capability has new requirements or a new status.
    CapabilityUpdated(stripe_shared::Capability),
    /// Occurs whenever there is a positive remaining cash balance after Stripe automatically reconciles new funds into the cash balance.
    /// If you enabled manual reconciliation, this webhook will fire whenever there are new funds into the cash balance.
    CashBalanceFundsAvailable(stripe_shared::CashBalance),
    /// Occurs whenever a previously uncaptured charge is captured.
    ChargeCaptured(stripe_shared::Charge),
    /// Occurs when a dispute is closed and the dispute status changes to `lost`, `warning_closed`, or `won`.
    ChargeDisputeClosed(stripe_shared::Dispute),
    /// Occurs whenever a customer disputes a charge with their bank.
    ChargeDisputeCreated(stripe_shared::Dispute),
    /// Occurs when funds are reinstated to your account after a dispute is closed.
    /// This includes [partially refunded payments](https://docs.stripe.com/disputes#disputes-on-partially-refunded-payments).
    ChargeDisputeFundsReinstated(stripe_shared::Dispute),
    /// Occurs when funds are removed from your account due to a dispute.
    ChargeDisputeFundsWithdrawn(stripe_shared::Dispute),
    /// Occurs when the dispute is updated (usually with evidence).
    ChargeDisputeUpdated(stripe_shared::Dispute),
    /// Occurs whenever an uncaptured charge expires.
    ChargeExpired(stripe_shared::Charge),
    /// Occurs whenever a failed charge attempt occurs.
    ChargeFailed(stripe_shared::Charge),
    /// Occurs whenever a pending charge is created.
    ChargePending(stripe_shared::Charge),
    /// Occurs whenever a refund is updated, on selected payment methods.
    ChargeRefundUpdated(stripe_shared::Refund),
    /// Occurs whenever a charge is refunded, including partial refunds.
    ChargeRefunded(stripe_shared::Charge),
    /// Occurs whenever a charge is successful.
    ChargeSucceeded(stripe_shared::Charge),
    /// Occurs whenever a charge description or metadata is updated, or upon an asynchronous capture.
    ChargeUpdated(stripe_shared::Charge),
    /// Occurs when a payment intent using a delayed payment method fails.
    #[cfg(feature = "stripe_checkout")]
    CheckoutSessionAsyncPaymentFailed(stripe_checkout::CheckoutSession),
    /// Occurs when a payment intent using a delayed payment method finally succeeds.
    #[cfg(feature = "stripe_checkout")]
    CheckoutSessionAsyncPaymentSucceeded(stripe_checkout::CheckoutSession),
    /// Occurs when a Checkout Session has been successfully completed.
    #[cfg(feature = "stripe_checkout")]
    CheckoutSessionCompleted(stripe_checkout::CheckoutSession),
    /// Occurs when a Checkout Session is expired.
    #[cfg(feature = "stripe_checkout")]
    CheckoutSessionExpired(stripe_checkout::CheckoutSession),
    /// Occurs when a Climate order is canceled.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderCanceled(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order is created.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderCreated(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order is delayed.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderDelayed(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order is delivered.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderDelivered(stripe_misc::ClimateOrder),
    /// Occurs when a Climate order's product is substituted for another.
    #[cfg(feature = "stripe_misc")]
    ClimateOrderProductSubstituted(stripe_misc::ClimateOrder),
    /// Occurs when a Climate product is created.
    #[cfg(feature = "stripe_misc")]
    ClimateProductCreated(stripe_misc::ClimateProduct),
    /// Occurs when a Climate product is updated.
    #[cfg(feature = "stripe_misc")]
    ClimateProductPricingUpdated(stripe_misc::ClimateProduct),
    /// Occurs whenever a coupon is created.
    CouponCreated(stripe_shared::Coupon),
    /// Occurs whenever a coupon is deleted.
    CouponDeleted(stripe_shared::Coupon),
    /// Occurs whenever a coupon is updated.
    CouponUpdated(stripe_shared::Coupon),
    /// Occurs whenever a credit note is created.
    CreditNoteCreated(stripe_shared::CreditNote),
    /// Occurs whenever a credit note is updated.
    CreditNoteUpdated(stripe_shared::CreditNote),
    /// Occurs whenever a credit note is voided.
    CreditNoteVoided(stripe_shared::CreditNote),
    /// Occurs whenever a new customer is created.
    CustomerCreated(stripe_shared::Customer),
    /// Occurs whenever a customer is deleted.
    CustomerDeleted(stripe_shared::Customer),
    /// Occurs whenever a coupon is attached to a customer.
    CustomerDiscountCreated(stripe_shared::Discount),
    /// Occurs whenever a coupon is removed from a customer.
    CustomerDiscountDeleted(stripe_shared::Discount),
    /// Occurs whenever a customer is switched from one coupon to another.
    CustomerDiscountUpdated(stripe_shared::Discount),
    /// Occurs whenever a new source is created for a customer.
    CustomerSourceCreated(CustomerSourceCreated),
    /// Occurs whenever a source is removed from a customer.
    CustomerSourceDeleted(CustomerSourceDeleted),
    /// Occurs whenever a card or source will expire at the end of the month.
    CustomerSourceExpiring(CustomerSourceExpiring),
    /// Occurs whenever a source's details are changed.
    CustomerSourceUpdated(CustomerSourceUpdated),
    /// Occurs whenever a customer is signed up for a new plan.
    CustomerSubscriptionCreated(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription ends.
    CustomerSubscriptionDeleted(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription is paused.
    /// Only applies when subscriptions enter `status=paused`, not when [payment collection](https://docs.stripe.com/billing/subscriptions/pause) is paused.
    CustomerSubscriptionPaused(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription's pending update is applied, and the subscription is updated.
    CustomerSubscriptionPendingUpdateApplied(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription's pending update expires before the related invoice is paid.
    CustomerSubscriptionPendingUpdateExpired(stripe_shared::Subscription),
    /// Occurs whenever a customer's subscription is no longer paused.
    /// Only applies when a `status=paused` subscription is [resumed](https://docs.stripe.com/api/subscriptions/resume), not when [payment collection](https://docs.stripe.com/billing/subscriptions/pause) is resumed.
    CustomerSubscriptionResumed(stripe_shared::Subscription),
    /// Occurs three days before a subscription's trial period is scheduled to end, or when a trial is ended immediately (using `trial_end=now`).
    CustomerSubscriptionTrialWillEnd(stripe_shared::Subscription),
    /// Occurs whenever a subscription changes (e.g., switching from one plan to another, or changing the status from trial to active).
    CustomerSubscriptionUpdated(stripe_shared::Subscription),
    /// Occurs whenever a tax ID is created for a customer.
    CustomerTaxIdCreated(stripe_shared::TaxId),
    /// Occurs whenever a tax ID is deleted from a customer.
    CustomerTaxIdDeleted(stripe_shared::TaxId),
    /// Occurs whenever a customer's tax ID is updated.
    CustomerTaxIdUpdated(stripe_shared::TaxId),
    /// Occurs whenever any property of a customer changes.
    CustomerUpdated(stripe_shared::Customer),
    /// Occurs whenever a new customer cash balance transactions is created.
    CustomerCashBalanceTransactionCreated(stripe_shared::CustomerCashBalanceTransaction),
    /// Occurs whenever a customer's entitlements change.
    #[cfg(feature = "stripe_misc")]
    EntitlementsActiveEntitlementSummaryUpdated(stripe_misc::EntitlementsActiveEntitlementSummary),
    /// Occurs whenever a new Stripe-generated file is available for your account.
    FileCreated(stripe_shared::File),
    /// Occurs when a new Financial Connections account is created.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountCreated(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when a Financial Connections account's status is updated from `active` to `inactive`.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountDeactivated(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when a Financial Connections account is disconnected.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountDisconnected(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when a Financial Connections account's status is updated from `inactive` to `active`.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountReactivated(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when an Account’s `balance_refresh` status transitions from `pending` to either `succeeded` or `failed`.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountRefreshedBalance(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when an Account’s `ownership_refresh` status transitions from `pending` to either `succeeded` or `failed`.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountRefreshedOwnership(stripe_misc::FinancialConnectionsAccount),
    /// Occurs when an Account’s `transaction_refresh` status transitions from `pending` to either `succeeded` or `failed`.
    #[cfg(feature = "stripe_misc")]
    FinancialConnectionsAccountRefreshedTransactions(stripe_misc::FinancialConnectionsAccount),
    /// Occurs whenever a VerificationSession is canceled
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionCanceled(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession is created
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionCreated(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession transitions to processing
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionProcessing(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession is redacted.
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionRedacted(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession transitions to require user input
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionRequiresInput(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a VerificationSession transitions to verified
    #[cfg(feature = "stripe_misc")]
    IdentityVerificationSessionVerified(stripe_misc::IdentityVerificationSession),
    /// Occurs whenever a new invoice is created.
    /// To learn how webhooks can be used with this event, and how they can affect it, see [Using Webhooks with Subscriptions](https://docs.stripe.com/subscriptions/webhooks).
    InvoiceCreated(stripe_shared::Invoice),
    /// Occurs whenever a draft invoice is deleted.
    /// Note: This event is not sent for [invoice previews](https://docs.stripe.com/api/invoices/create_preview).
    InvoiceDeleted(stripe_shared::Invoice),
    /// Occurs whenever a draft invoice cannot be finalized.
    /// See the invoice’s [last finalization error](https://docs.stripe.com/api/invoices/object#invoice_object-last_finalization_error) for details.
    InvoiceFinalizationFailed(stripe_shared::Invoice),
    /// Occurs whenever a draft invoice is finalized and updated to be an open invoice.
    InvoiceFinalized(stripe_shared::Invoice),
    /// Occurs whenever an invoice is marked uncollectible.
    InvoiceMarkedUncollectible(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt succeeds or an invoice is marked as paid out-of-band.
    InvoicePaid(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt requires further user action to complete.
    InvoicePaymentActionRequired(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt fails, due either to a declined payment or to the lack of a stored payment method.
    InvoicePaymentFailed(stripe_shared::Invoice),
    /// Occurs whenever an invoice payment attempt succeeds.
    InvoicePaymentSucceeded(stripe_shared::Invoice),
    /// Occurs whenever an invoice email is sent out.
    InvoiceSent(stripe_shared::Invoice),
    /// Occurs X number of days before a subscription is scheduled to create an invoice that is automatically charged&mdash;where X is determined by your [subscriptions settings](https://dashboard.stripe.com/account/billing/automatic).
    /// Note: The received `Invoice` object will not have an invoice ID.
    InvoiceUpcoming(stripe_shared::Invoice),
    /// Occurs whenever an invoice changes (e.g., the invoice amount).
    InvoiceUpdated(stripe_shared::Invoice),
    /// Occurs whenever an invoice is voided.
    InvoiceVoided(stripe_shared::Invoice),
    /// Occurs whenever an invoice item is created.
    InvoiceitemCreated(stripe_shared::InvoiceItem),
    /// Occurs whenever an invoice item is deleted.
    InvoiceitemDeleted(stripe_shared::InvoiceItem),
    /// Occurs whenever an authorization is created.
    IssuingAuthorizationCreated(stripe_shared::IssuingAuthorization),
    /// Represents a synchronous request for authorization, see [Using your integration to handle authorization requests](https://docs.stripe.com/issuing/purchases/authorizations#authorization-handling).
    IssuingAuthorizationRequest(stripe_shared::IssuingAuthorization),
    /// Occurs whenever an authorization is updated.
    IssuingAuthorizationUpdated(stripe_shared::IssuingAuthorization),
    /// Occurs whenever a card is created.
    IssuingCardCreated(stripe_shared::IssuingCard),
    /// Occurs whenever a card is updated.
    IssuingCardUpdated(stripe_shared::IssuingCard),
    /// Occurs whenever a cardholder is created.
    IssuingCardholderCreated(stripe_shared::IssuingCardholder),
    /// Occurs whenever a cardholder is updated.
    IssuingCardholderUpdated(stripe_shared::IssuingCardholder),
    /// Occurs whenever a dispute is won, lost or expired.
    IssuingDisputeClosed(stripe_shared::IssuingDispute),
    /// Occurs whenever a dispute is created.
    IssuingDisputeCreated(stripe_shared::IssuingDispute),
    /// Occurs whenever funds are reinstated to your account for an Issuing dispute.
    IssuingDisputeFundsReinstated(stripe_shared::IssuingDispute),
    /// Occurs whenever a dispute is submitted.
    IssuingDisputeSubmitted(stripe_shared::IssuingDispute),
    /// Occurs whenever a dispute is updated.
    IssuingDisputeUpdated(stripe_shared::IssuingDispute),
    /// Occurs whenever an issuing digital wallet token is created.
    IssuingTokenCreated(stripe_shared::IssuingToken),
    /// Occurs whenever an issuing digital wallet token is updated.
    IssuingTokenUpdated(stripe_shared::IssuingToken),
    /// Occurs whenever an issuing transaction is created.
    IssuingTransactionCreated(stripe_shared::IssuingTransaction),
    /// Occurs whenever an issuing transaction is updated.
    IssuingTransactionUpdated(stripe_shared::IssuingTransaction),
    /// Occurs whenever a Mandate is updated.
    MandateUpdated(stripe_shared::Mandate),
    /// Occurs when a PaymentIntent has funds to be captured.
    /// Check the `amount_capturable` property on the PaymentIntent to determine the amount that can be captured.
    /// You may capture the PaymentIntent with an `amount_to_capture` value up to the specified amount.
    /// [Learn more about capturing PaymentIntents.](https://docs.stripe.com/api/payment_intents/capture).
    PaymentIntentAmountCapturableUpdated(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent is canceled.
    PaymentIntentCanceled(stripe_shared::PaymentIntent),
    /// Occurs when a new PaymentIntent is created.
    PaymentIntentCreated(stripe_shared::PaymentIntent),
    /// Occurs when funds are applied to a customer_balance PaymentIntent and the 'amount_remaining' changes.
    PaymentIntentPartiallyFunded(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent has failed the attempt to create a payment method or a payment.
    PaymentIntentPaymentFailed(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent has started processing.
    PaymentIntentProcessing(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent transitions to requires_action state
    PaymentIntentRequiresAction(stripe_shared::PaymentIntent),
    /// Occurs when a PaymentIntent has successfully completed payment.
    PaymentIntentSucceeded(stripe_shared::PaymentIntent),
    /// Occurs when a payment link is created.
    PaymentLinkCreated(stripe_shared::PaymentLink),
    /// Occurs when a payment link is updated.
    PaymentLinkUpdated(stripe_shared::PaymentLink),
    /// Occurs whenever a new payment method is attached to a customer.
    PaymentMethodAttached(stripe_shared::PaymentMethod),
    /// Occurs whenever a payment method's details are automatically updated by the network.
    PaymentMethodAutomaticallyUpdated(stripe_shared::PaymentMethod),
    /// Occurs whenever a payment method is detached from a customer.
    PaymentMethodDetached(stripe_shared::PaymentMethod),
    /// Occurs whenever a payment method is updated via the [PaymentMethod update API](https://docs.stripe.com/api/payment_methods/update).
    PaymentMethodUpdated(stripe_shared::PaymentMethod),
    /// Occurs whenever a payout is canceled.
    PayoutCanceled(stripe_shared::Payout),
    /// Occurs whenever a payout is created.
    PayoutCreated(stripe_shared::Payout),
    /// Occurs whenever a payout attempt fails.
    PayoutFailed(stripe_shared::Payout),
    /// Occurs whenever a payout is *expected* to be available in the destination account.
    /// If the payout fails, a `payout.failed` notification is also sent, at a later time.
    PayoutPaid(stripe_shared::Payout),
    /// Occurs whenever balance transactions paid out in an automatic payout can be queried.
    PayoutReconciliationCompleted(stripe_shared::Payout),
    /// Occurs whenever a payout is updated.
    PayoutUpdated(stripe_shared::Payout),
    /// Occurs whenever a person associated with an account is created.
    PersonCreated(stripe_shared::Person),
    /// Occurs whenever a person associated with an account is deleted.
    PersonDeleted(stripe_shared::Person),
    /// Occurs whenever a person associated with an account is updated.
    PersonUpdated(stripe_shared::Person),
    /// Occurs whenever a plan is created.
    PlanCreated(stripe_shared::Plan),
    /// Occurs whenever a plan is deleted.
    PlanDeleted(stripe_shared::Plan),
    /// Occurs whenever a plan is updated.
    PlanUpdated(stripe_shared::Plan),
    /// Occurs whenever a price is created.
    PriceCreated(stripe_shared::Price),
    /// Occurs whenever a price is deleted.
    PriceDeleted(stripe_shared::Price),
    /// Occurs whenever a price is updated.
    PriceUpdated(stripe_shared::Price),
    /// Occurs whenever a product is created.
    ProductCreated(stripe_shared::Product),
    /// Occurs whenever a product is deleted.
    ProductDeleted(stripe_shared::Product),
    /// Occurs whenever a product is updated.
    ProductUpdated(stripe_shared::Product),
    /// Occurs whenever a promotion code is created.
    PromotionCodeCreated(stripe_shared::PromotionCode),
    /// Occurs whenever a promotion code is updated.
    PromotionCodeUpdated(stripe_shared::PromotionCode),
    /// Occurs whenever a quote is accepted.
    QuoteAccepted(stripe_shared::Quote),
    /// Occurs whenever a quote is canceled.
    QuoteCanceled(stripe_shared::Quote),
    /// Occurs whenever a quote is created.
    QuoteCreated(stripe_shared::Quote),
    /// Occurs whenever a quote is finalized.
    QuoteFinalized(stripe_shared::Quote),
    /// Occurs whenever an early fraud warning is created.
    #[cfg(feature = "stripe_fraud")]
    RadarEarlyFraudWarningCreated(stripe_fraud::RadarEarlyFraudWarning),
    /// Occurs whenever an early fraud warning is updated.
    #[cfg(feature = "stripe_fraud")]
    RadarEarlyFraudWarningUpdated(stripe_fraud::RadarEarlyFraudWarning),
    /// Occurs whenever a refund from a customer's cash balance is created.
    RefundCreated(stripe_shared::Refund),
    /// Occurs whenever a refund from a customer's cash balance is updated.
    RefundUpdated(stripe_shared::Refund),
    /// Occurs whenever a requested `ReportRun` failed to complete.
    #[cfg(feature = "stripe_misc")]
    ReportingReportRunFailed(stripe_misc::ReportingReportRun),
    /// Occurs whenever a requested `ReportRun` completed successfully.
    #[cfg(feature = "stripe_misc")]
    ReportingReportRunSucceeded(stripe_misc::ReportingReportRun),
    /// Occurs whenever a `ReportType` is updated (typically to indicate that a new day's data has come available).
    #[cfg(feature = "stripe_misc")]
    ReportingReportTypeUpdated(stripe_misc::ReportingReportType),
    /// Occurs whenever a review is closed.
    /// The review's `reason` field indicates why: `approved`, `disputed`, `refunded`, or `refunded_as_fraud`.
    ReviewClosed(stripe_shared::Review),
    /// Occurs whenever a review is opened.
    ReviewOpened(stripe_shared::Review),
    /// Occurs when a SetupIntent is canceled.
    SetupIntentCanceled(stripe_shared::SetupIntent),
    /// Occurs when a new SetupIntent is created.
    SetupIntentCreated(stripe_shared::SetupIntent),
    /// Occurs when a SetupIntent is in requires_action state.
    SetupIntentRequiresAction(stripe_shared::SetupIntent),
    /// Occurs when a SetupIntent has failed the attempt to setup a payment method.
    SetupIntentSetupFailed(stripe_shared::SetupIntent),
    /// Occurs when an SetupIntent has successfully setup a payment method.
    SetupIntentSucceeded(stripe_shared::SetupIntent),
    /// Occurs whenever a Sigma scheduled query run finishes.
    #[cfg(feature = "stripe_misc")]
    SigmaScheduledQueryRunCreated(stripe_misc::ScheduledQueryRun),
    /// Occurs whenever a source is canceled.
    SourceCanceled(stripe_shared::Source),
    /// Occurs whenever a source transitions to chargeable.
    SourceChargeable(stripe_shared::Source),
    /// Occurs whenever a source fails.
    SourceFailed(stripe_shared::Source),
    /// Occurs whenever a source mandate notification method is set to manual.
    #[cfg(feature = "stripe_payment")]
    SourceMandateNotification(stripe_payment::SourceMandateNotification),
    /// Occurs whenever the refund attributes are required on a receiver source to process a refund or a mispayment.
    SourceRefundAttributesRequired(stripe_shared::Source),
    /// Occurs whenever a source transaction is created.
    SourceTransactionCreated(stripe_shared::SourceTransaction),
    /// Occurs whenever a source transaction is updated.
    SourceTransactionUpdated(stripe_shared::SourceTransaction),
    /// Occurs whenever a subscription schedule is canceled due to the underlying subscription being canceled because of delinquency.
    SubscriptionScheduleAborted(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a subscription schedule is canceled.
    SubscriptionScheduleCanceled(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a new subscription schedule is completed.
    SubscriptionScheduleCompleted(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a new subscription schedule is created.
    SubscriptionScheduleCreated(stripe_shared::SubscriptionSchedule),
    /// Occurs 7 days before a subscription schedule will expire.
    SubscriptionScheduleExpiring(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a new subscription schedule is released.
    SubscriptionScheduleReleased(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever a subscription schedule is updated.
    SubscriptionScheduleUpdated(stripe_shared::SubscriptionSchedule),
    /// Occurs whenever tax settings is updated.
    #[cfg(feature = "stripe_misc")]
    TaxSettingsUpdated(stripe_misc::TaxSettings),
    /// Occurs whenever a new tax rate is created.
    TaxRateCreated(stripe_shared::TaxRate),
    /// Occurs whenever a tax rate is updated.
    TaxRateUpdated(stripe_shared::TaxRate),
    /// Occurs whenever an action sent to a Terminal reader failed.
    #[cfg(feature = "stripe_terminal")]
    TerminalReaderActionFailed(stripe_terminal::TerminalReader),
    /// Occurs whenever an action sent to a Terminal reader was successful.
    #[cfg(feature = "stripe_terminal")]
    TerminalReaderActionSucceeded(stripe_terminal::TerminalReader),
    /// Occurs whenever a test clock starts advancing.
    TestHelpersTestClockAdvancing(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock is created.
    TestHelpersTestClockCreated(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock is deleted.
    TestHelpersTestClockDeleted(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock fails to advance its frozen time.
    TestHelpersTestClockInternalFailure(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a test clock transitions to a ready status.
    TestHelpersTestClockReady(stripe_shared::TestHelpersTestClock),
    /// Occurs whenever a top-up is canceled.
    TopupCanceled(stripe_shared::Topup),
    /// Occurs whenever a top-up is created.
    TopupCreated(stripe_shared::Topup),
    /// Occurs whenever a top-up fails.
    TopupFailed(stripe_shared::Topup),
    /// Occurs whenever a top-up is reversed.
    TopupReversed(stripe_shared::Topup),
    /// Occurs whenever a top-up succeeds.
    TopupSucceeded(stripe_shared::Topup),
    /// Occurs whenever a transfer is created.
    TransferCreated(stripe_shared::Transfer),
    /// Occurs whenever a transfer is reversed, including partial reversals.
    TransferReversed(stripe_shared::Transfer),
    /// Occurs whenever a transfer's description or metadata is updated.
    TransferUpdated(stripe_shared::Transfer),
    /// Occurs whenever an CreditReversal is submitted and created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryCreditReversalCreated(stripe_treasury::TreasuryCreditReversal),
    /// Occurs whenever an CreditReversal post is posted.
    #[cfg(feature = "stripe_treasury")]
    TreasuryCreditReversalPosted(stripe_treasury::TreasuryCreditReversal),
    /// Occurs whenever a DebitReversal is completed.
    #[cfg(feature = "stripe_treasury")]
    TreasuryDebitReversalCompleted(stripe_treasury::TreasuryDebitReversal),
    /// Occurs whenever a DebitReversal is created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryDebitReversalCreated(stripe_treasury::TreasuryDebitReversal),
    /// Occurs whenever an initial credit is granted on a DebitReversal.
    #[cfg(feature = "stripe_treasury")]
    TreasuryDebitReversalInitialCreditGranted(stripe_treasury::TreasuryDebitReversal),
    /// Occurs whenever the status of the FinancialAccount becomes closed.
    #[cfg(feature = "stripe_treasury")]
    TreasuryFinancialAccountClosed(stripe_treasury::TreasuryFinancialAccount),
    /// Occurs whenever a new FinancialAccount is created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryFinancialAccountCreated(stripe_treasury::TreasuryFinancialAccount),
    /// Occurs whenever the statuses of any features within an existing FinancialAccount are updated.
    #[cfg(feature = "stripe_treasury")]
    TreasuryFinancialAccountFeaturesStatusUpdated(stripe_treasury::TreasuryFinancialAccount),
    /// Occurs whenever an InboundTransfer is canceled.
    #[cfg(feature = "stripe_treasury")]
    TreasuryInboundTransferCanceled(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an InboundTransfer is created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryInboundTransferCreated(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an InboundTransfer has failed.
    #[cfg(feature = "stripe_treasury")]
    TreasuryInboundTransferFailed(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an InboundTransfer has succeeded.
    #[cfg(feature = "stripe_treasury")]
    TreasuryInboundTransferSucceeded(stripe_treasury::TreasuryInboundTransfer),
    /// Occurs whenever an OutboundPayment is canceled.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentCanceled(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever a new OutboundPayment is successfully created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentCreated(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever the arrival date on an OutboundPayment updates.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentExpectedArrivalDateUpdated(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundPayment fails.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentFailed(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundPayment posts.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentPosted(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundPayment was returned.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundPaymentReturned(stripe_treasury::TreasuryOutboundPayment),
    /// Occurs whenever an OutboundTransfer is canceled.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferCanceled(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer is created.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferCreated(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever the arrival date on an OutboundTransfer updates.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferExpectedArrivalDateUpdated(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer has failed.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferFailed(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer is posted.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferPosted(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever an OutboundTransfer is returned.
    #[cfg(feature = "stripe_treasury")]
    TreasuryOutboundTransferReturned(stripe_treasury::TreasuryOutboundTransfer),
    /// Occurs whenever a received_credit is created as a result of funds being pushed by another account.
    #[cfg(feature = "stripe_treasury")]
    TreasuryReceivedCreditCreated(stripe_treasury::TreasuryReceivedCredit),
    /// Occurs whenever a received_credit transitions to failed state. Only applicable for check deposits.
    #[cfg(feature = "stripe_treasury")]
    TreasuryReceivedCreditFailed(stripe_treasury::TreasuryReceivedCredit),
    /// Occurs whenever a received_credit transitions to succeeded state.
    /// Only applicable for check deposits.
    #[cfg(feature = "stripe_treasury")]
    TreasuryReceivedCreditSucceeded(stripe_treasury::TreasuryReceivedCredit),
    /// Occurs whenever a received_debit is created as a result of funds being pulled by another account.
    #[cfg(feature = "stripe_treasury")]
    TreasuryReceivedDebitCreated(stripe_treasury::TreasuryReceivedDebit),
    Unknown(miniserde::json::Value),
}
impl EventObject {
    pub(crate) fn from_raw_data(typ: &str, data: miniserde::json::Value) -> Option<Self> {
        use stripe_types::miniserde_helpers::FromValueOpt;
        Some(match typ {
            "account.application.authorized" => {
                Self::AccountApplicationAuthorized(FromValueOpt::from_value(data)?)
            }
            "account.application.deauthorized" => {
                Self::AccountApplicationDeauthorized(FromValueOpt::from_value(data)?)
            }
            "account.external_account.created" => {
                Self::AccountExternalAccountCreated(FromValueOpt::from_value(data)?)
            }
            "account.external_account.deleted" => {
                Self::AccountExternalAccountDeleted(FromValueOpt::from_value(data)?)
            }
            "account.external_account.updated" => {
                Self::AccountExternalAccountUpdated(FromValueOpt::from_value(data)?)
            }
            "account.updated" => Self::AccountUpdated(FromValueOpt::from_value(data)?),
            "application_fee.created" => {
                Self::ApplicationFeeCreated(FromValueOpt::from_value(data)?)
            }
            "application_fee.refund.updated" => {
                Self::ApplicationFeeRefundUpdated(FromValueOpt::from_value(data)?)
            }
            "application_fee.refunded" => {
                Self::ApplicationFeeRefunded(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_core")]
            "balance.available" => Self::BalanceAvailable(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_billing")]
            "billing_portal.configuration.created" => {
                Self::BillingPortalConfigurationCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_billing")]
            "billing_portal.configuration.updated" => {
                Self::BillingPortalConfigurationUpdated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_billing")]
            "billing_portal.session.created" => {
                Self::BillingPortalSessionCreated(FromValueOpt::from_value(data)?)
            }
            "capability.updated" => Self::CapabilityUpdated(FromValueOpt::from_value(data)?),
            "cash_balance.funds_available" => {
                Self::CashBalanceFundsAvailable(FromValueOpt::from_value(data)?)
            }
            "charge.captured" => Self::ChargeCaptured(FromValueOpt::from_value(data)?),
            "charge.dispute.closed" => Self::ChargeDisputeClosed(FromValueOpt::from_value(data)?),
            "charge.dispute.created" => Self::ChargeDisputeCreated(FromValueOpt::from_value(data)?),
            "charge.dispute.funds_reinstated" => {
                Self::ChargeDisputeFundsReinstated(FromValueOpt::from_value(data)?)
            }
            "charge.dispute.funds_withdrawn" => {
                Self::ChargeDisputeFundsWithdrawn(FromValueOpt::from_value(data)?)
            }
            "charge.dispute.updated" => Self::ChargeDisputeUpdated(FromValueOpt::from_value(data)?),
            "charge.expired" => Self::ChargeExpired(FromValueOpt::from_value(data)?),
            "charge.failed" => Self::ChargeFailed(FromValueOpt::from_value(data)?),
            "charge.pending" => Self::ChargePending(FromValueOpt::from_value(data)?),
            "charge.refund.updated" => Self::ChargeRefundUpdated(FromValueOpt::from_value(data)?),
            "charge.refunded" => Self::ChargeRefunded(FromValueOpt::from_value(data)?),
            "charge.succeeded" => Self::ChargeSucceeded(FromValueOpt::from_value(data)?),
            "charge.updated" => Self::ChargeUpdated(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_checkout")]
            "checkout.session.async_payment_failed" => {
                Self::CheckoutSessionAsyncPaymentFailed(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_checkout")]
            "checkout.session.async_payment_succeeded" => {
                Self::CheckoutSessionAsyncPaymentSucceeded(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_checkout")]
            "checkout.session.completed" => {
                Self::CheckoutSessionCompleted(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_checkout")]
            "checkout.session.expired" => {
                Self::CheckoutSessionExpired(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.order.canceled" => Self::ClimateOrderCanceled(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_misc")]
            "climate.order.created" => Self::ClimateOrderCreated(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_misc")]
            "climate.order.delayed" => Self::ClimateOrderDelayed(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_misc")]
            "climate.order.delivered" => {
                Self::ClimateOrderDelivered(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.order.product_substituted" => {
                Self::ClimateOrderProductSubstituted(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.product.created" => {
                Self::ClimateProductCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "climate.product.pricing_updated" => {
                Self::ClimateProductPricingUpdated(FromValueOpt::from_value(data)?)
            }
            "coupon.created" => Self::CouponCreated(FromValueOpt::from_value(data)?),
            "coupon.deleted" => Self::CouponDeleted(FromValueOpt::from_value(data)?),
            "coupon.updated" => Self::CouponUpdated(FromValueOpt::from_value(data)?),
            "credit_note.created" => Self::CreditNoteCreated(FromValueOpt::from_value(data)?),
            "credit_note.updated" => Self::CreditNoteUpdated(FromValueOpt::from_value(data)?),
            "credit_note.voided" => Self::CreditNoteVoided(FromValueOpt::from_value(data)?),
            "customer.created" => Self::CustomerCreated(FromValueOpt::from_value(data)?),
            "customer.deleted" => Self::CustomerDeleted(FromValueOpt::from_value(data)?),
            "customer.discount.created" => {
                Self::CustomerDiscountCreated(FromValueOpt::from_value(data)?)
            }
            "customer.discount.deleted" => {
                Self::CustomerDiscountDeleted(FromValueOpt::from_value(data)?)
            }
            "customer.discount.updated" => {
                Self::CustomerDiscountUpdated(FromValueOpt::from_value(data)?)
            }
            "customer.source.created" => {
                Self::CustomerSourceCreated(FromValueOpt::from_value(data)?)
            }
            "customer.source.deleted" => {
                Self::CustomerSourceDeleted(FromValueOpt::from_value(data)?)
            }
            "customer.source.expiring" => {
                Self::CustomerSourceExpiring(FromValueOpt::from_value(data)?)
            }
            "customer.source.updated" => {
                Self::CustomerSourceUpdated(FromValueOpt::from_value(data)?)
            }
            "customer.subscription.created" => {
                Self::CustomerSubscriptionCreated(FromValueOpt::from_value(data)?)
            }
            "customer.subscription.deleted" => {
                Self::CustomerSubscriptionDeleted(FromValueOpt::from_value(data)?)
            }
            "customer.subscription.paused" => {
                Self::CustomerSubscriptionPaused(FromValueOpt::from_value(data)?)
            }
            "customer.subscription.pending_update_applied" => {
                Self::CustomerSubscriptionPendingUpdateApplied(FromValueOpt::from_value(data)?)
            }
            "customer.subscription.pending_update_expired" => {
                Self::CustomerSubscriptionPendingUpdateExpired(FromValueOpt::from_value(data)?)
            }
            "customer.subscription.resumed" => {
                Self::CustomerSubscriptionResumed(FromValueOpt::from_value(data)?)
            }
            "customer.subscription.trial_will_end" => {
                Self::CustomerSubscriptionTrialWillEnd(FromValueOpt::from_value(data)?)
            }
            "customer.subscription.updated" => {
                Self::CustomerSubscriptionUpdated(FromValueOpt::from_value(data)?)
            }
            "customer.tax_id.created" => {
                Self::CustomerTaxIdCreated(FromValueOpt::from_value(data)?)
            }
            "customer.tax_id.deleted" => {
                Self::CustomerTaxIdDeleted(FromValueOpt::from_value(data)?)
            }
            "customer.tax_id.updated" => {
                Self::CustomerTaxIdUpdated(FromValueOpt::from_value(data)?)
            }
            "customer.updated" => Self::CustomerUpdated(FromValueOpt::from_value(data)?),
            "customer_cash_balance_transaction.created" => {
                Self::CustomerCashBalanceTransactionCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "entitlements.active_entitlement_summary.updated" => {
                Self::EntitlementsActiveEntitlementSummaryUpdated(FromValueOpt::from_value(data)?)
            }
            "file.created" => Self::FileCreated(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.created" => {
                Self::FinancialConnectionsAccountCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.deactivated" => {
                Self::FinancialConnectionsAccountDeactivated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.disconnected" => {
                Self::FinancialConnectionsAccountDisconnected(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.reactivated" => {
                Self::FinancialConnectionsAccountReactivated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.refreshed_balance" => {
                Self::FinancialConnectionsAccountRefreshedBalance(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.refreshed_ownership" => {
                Self::FinancialConnectionsAccountRefreshedOwnership(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "financial_connections.account.refreshed_transactions" => {
                Self::FinancialConnectionsAccountRefreshedTransactions(FromValueOpt::from_value(
                    data,
                )?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.canceled" => {
                Self::IdentityVerificationSessionCanceled(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.created" => {
                Self::IdentityVerificationSessionCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.processing" => {
                Self::IdentityVerificationSessionProcessing(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.redacted" => {
                Self::IdentityVerificationSessionRedacted(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.requires_input" => {
                Self::IdentityVerificationSessionRequiresInput(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "identity.verification_session.verified" => {
                Self::IdentityVerificationSessionVerified(FromValueOpt::from_value(data)?)
            }
            "invoice.created" => Self::InvoiceCreated(FromValueOpt::from_value(data)?),
            "invoice.deleted" => Self::InvoiceDeleted(FromValueOpt::from_value(data)?),
            "invoice.finalization_failed" => {
                Self::InvoiceFinalizationFailed(FromValueOpt::from_value(data)?)
            }
            "invoice.finalized" => Self::InvoiceFinalized(FromValueOpt::from_value(data)?),
            "invoice.marked_uncollectible" => {
                Self::InvoiceMarkedUncollectible(FromValueOpt::from_value(data)?)
            }
            "invoice.paid" => Self::InvoicePaid(FromValueOpt::from_value(data)?),
            "invoice.payment_action_required" => {
                Self::InvoicePaymentActionRequired(FromValueOpt::from_value(data)?)
            }
            "invoice.payment_failed" => Self::InvoicePaymentFailed(FromValueOpt::from_value(data)?),
            "invoice.payment_succeeded" => {
                Self::InvoicePaymentSucceeded(FromValueOpt::from_value(data)?)
            }
            "invoice.sent" => Self::InvoiceSent(FromValueOpt::from_value(data)?),
            "invoice.upcoming" => Self::InvoiceUpcoming(FromValueOpt::from_value(data)?),
            "invoice.updated" => Self::InvoiceUpdated(FromValueOpt::from_value(data)?),
            "invoice.voided" => Self::InvoiceVoided(FromValueOpt::from_value(data)?),
            "invoiceitem.created" => Self::InvoiceitemCreated(FromValueOpt::from_value(data)?),
            "invoiceitem.deleted" => Self::InvoiceitemDeleted(FromValueOpt::from_value(data)?),
            "issuing_authorization.created" => {
                Self::IssuingAuthorizationCreated(FromValueOpt::from_value(data)?)
            }
            "issuing_authorization.request" => {
                Self::IssuingAuthorizationRequest(FromValueOpt::from_value(data)?)
            }
            "issuing_authorization.updated" => {
                Self::IssuingAuthorizationUpdated(FromValueOpt::from_value(data)?)
            }
            "issuing_card.created" => Self::IssuingCardCreated(FromValueOpt::from_value(data)?),
            "issuing_card.updated" => Self::IssuingCardUpdated(FromValueOpt::from_value(data)?),
            "issuing_cardholder.created" => {
                Self::IssuingCardholderCreated(FromValueOpt::from_value(data)?)
            }
            "issuing_cardholder.updated" => {
                Self::IssuingCardholderUpdated(FromValueOpt::from_value(data)?)
            }
            "issuing_dispute.closed" => Self::IssuingDisputeClosed(FromValueOpt::from_value(data)?),
            "issuing_dispute.created" => {
                Self::IssuingDisputeCreated(FromValueOpt::from_value(data)?)
            }
            "issuing_dispute.funds_reinstated" => {
                Self::IssuingDisputeFundsReinstated(FromValueOpt::from_value(data)?)
            }
            "issuing_dispute.submitted" => {
                Self::IssuingDisputeSubmitted(FromValueOpt::from_value(data)?)
            }
            "issuing_dispute.updated" => {
                Self::IssuingDisputeUpdated(FromValueOpt::from_value(data)?)
            }
            "issuing_token.created" => Self::IssuingTokenCreated(FromValueOpt::from_value(data)?),
            "issuing_token.updated" => Self::IssuingTokenUpdated(FromValueOpt::from_value(data)?),
            "issuing_transaction.created" => {
                Self::IssuingTransactionCreated(FromValueOpt::from_value(data)?)
            }
            "issuing_transaction.updated" => {
                Self::IssuingTransactionUpdated(FromValueOpt::from_value(data)?)
            }
            "mandate.updated" => Self::MandateUpdated(FromValueOpt::from_value(data)?),
            "payment_intent.amount_capturable_updated" => {
                Self::PaymentIntentAmountCapturableUpdated(FromValueOpt::from_value(data)?)
            }
            "payment_intent.canceled" => {
                Self::PaymentIntentCanceled(FromValueOpt::from_value(data)?)
            }
            "payment_intent.created" => Self::PaymentIntentCreated(FromValueOpt::from_value(data)?),
            "payment_intent.partially_funded" => {
                Self::PaymentIntentPartiallyFunded(FromValueOpt::from_value(data)?)
            }
            "payment_intent.payment_failed" => {
                Self::PaymentIntentPaymentFailed(FromValueOpt::from_value(data)?)
            }
            "payment_intent.processing" => {
                Self::PaymentIntentProcessing(FromValueOpt::from_value(data)?)
            }
            "payment_intent.requires_action" => {
                Self::PaymentIntentRequiresAction(FromValueOpt::from_value(data)?)
            }
            "payment_intent.succeeded" => {
                Self::PaymentIntentSucceeded(FromValueOpt::from_value(data)?)
            }
            "payment_link.created" => Self::PaymentLinkCreated(FromValueOpt::from_value(data)?),
            "payment_link.updated" => Self::PaymentLinkUpdated(FromValueOpt::from_value(data)?),
            "payment_method.attached" => {
                Self::PaymentMethodAttached(FromValueOpt::from_value(data)?)
            }
            "payment_method.automatically_updated" => {
                Self::PaymentMethodAutomaticallyUpdated(FromValueOpt::from_value(data)?)
            }
            "payment_method.detached" => {
                Self::PaymentMethodDetached(FromValueOpt::from_value(data)?)
            }
            "payment_method.updated" => Self::PaymentMethodUpdated(FromValueOpt::from_value(data)?),
            "payout.canceled" => Self::PayoutCanceled(FromValueOpt::from_value(data)?),
            "payout.created" => Self::PayoutCreated(FromValueOpt::from_value(data)?),
            "payout.failed" => Self::PayoutFailed(FromValueOpt::from_value(data)?),
            "payout.paid" => Self::PayoutPaid(FromValueOpt::from_value(data)?),
            "payout.reconciliation_completed" => {
                Self::PayoutReconciliationCompleted(FromValueOpt::from_value(data)?)
            }
            "payout.updated" => Self::PayoutUpdated(FromValueOpt::from_value(data)?),
            "person.created" => Self::PersonCreated(FromValueOpt::from_value(data)?),
            "person.deleted" => Self::PersonDeleted(FromValueOpt::from_value(data)?),
            "person.updated" => Self::PersonUpdated(FromValueOpt::from_value(data)?),
            "plan.created" => Self::PlanCreated(FromValueOpt::from_value(data)?),
            "plan.deleted" => Self::PlanDeleted(FromValueOpt::from_value(data)?),
            "plan.updated" => Self::PlanUpdated(FromValueOpt::from_value(data)?),
            "price.created" => Self::PriceCreated(FromValueOpt::from_value(data)?),
            "price.deleted" => Self::PriceDeleted(FromValueOpt::from_value(data)?),
            "price.updated" => Self::PriceUpdated(FromValueOpt::from_value(data)?),
            "product.created" => Self::ProductCreated(FromValueOpt::from_value(data)?),
            "product.deleted" => Self::ProductDeleted(FromValueOpt::from_value(data)?),
            "product.updated" => Self::ProductUpdated(FromValueOpt::from_value(data)?),
            "promotion_code.created" => Self::PromotionCodeCreated(FromValueOpt::from_value(data)?),
            "promotion_code.updated" => Self::PromotionCodeUpdated(FromValueOpt::from_value(data)?),
            "quote.accepted" => Self::QuoteAccepted(FromValueOpt::from_value(data)?),
            "quote.canceled" => Self::QuoteCanceled(FromValueOpt::from_value(data)?),
            "quote.created" => Self::QuoteCreated(FromValueOpt::from_value(data)?),
            "quote.finalized" => Self::QuoteFinalized(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_fraud")]
            "radar.early_fraud_warning.created" => {
                Self::RadarEarlyFraudWarningCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_fraud")]
            "radar.early_fraud_warning.updated" => {
                Self::RadarEarlyFraudWarningUpdated(FromValueOpt::from_value(data)?)
            }
            "refund.created" => Self::RefundCreated(FromValueOpt::from_value(data)?),
            "refund.updated" => Self::RefundUpdated(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_misc")]
            "reporting.report_run.failed" => {
                Self::ReportingReportRunFailed(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "reporting.report_run.succeeded" => {
                Self::ReportingReportRunSucceeded(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "reporting.report_type.updated" => {
                Self::ReportingReportTypeUpdated(FromValueOpt::from_value(data)?)
            }
            "review.closed" => Self::ReviewClosed(FromValueOpt::from_value(data)?),
            "review.opened" => Self::ReviewOpened(FromValueOpt::from_value(data)?),
            "setup_intent.canceled" => Self::SetupIntentCanceled(FromValueOpt::from_value(data)?),
            "setup_intent.created" => Self::SetupIntentCreated(FromValueOpt::from_value(data)?),
            "setup_intent.requires_action" => {
                Self::SetupIntentRequiresAction(FromValueOpt::from_value(data)?)
            }
            "setup_intent.setup_failed" => {
                Self::SetupIntentSetupFailed(FromValueOpt::from_value(data)?)
            }
            "setup_intent.succeeded" => Self::SetupIntentSucceeded(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_misc")]
            "sigma.scheduled_query_run.created" => {
                Self::SigmaScheduledQueryRunCreated(FromValueOpt::from_value(data)?)
            }
            "source.canceled" => Self::SourceCanceled(FromValueOpt::from_value(data)?),
            "source.chargeable" => Self::SourceChargeable(FromValueOpt::from_value(data)?),
            "source.failed" => Self::SourceFailed(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_payment")]
            "source.mandate_notification" => {
                Self::SourceMandateNotification(FromValueOpt::from_value(data)?)
            }
            "source.refund_attributes_required" => {
                Self::SourceRefundAttributesRequired(FromValueOpt::from_value(data)?)
            }
            "source.transaction.created" => {
                Self::SourceTransactionCreated(FromValueOpt::from_value(data)?)
            }
            "source.transaction.updated" => {
                Self::SourceTransactionUpdated(FromValueOpt::from_value(data)?)
            }
            "subscription_schedule.aborted" => {
                Self::SubscriptionScheduleAborted(FromValueOpt::from_value(data)?)
            }
            "subscription_schedule.canceled" => {
                Self::SubscriptionScheduleCanceled(FromValueOpt::from_value(data)?)
            }
            "subscription_schedule.completed" => {
                Self::SubscriptionScheduleCompleted(FromValueOpt::from_value(data)?)
            }
            "subscription_schedule.created" => {
                Self::SubscriptionScheduleCreated(FromValueOpt::from_value(data)?)
            }
            "subscription_schedule.expiring" => {
                Self::SubscriptionScheduleExpiring(FromValueOpt::from_value(data)?)
            }
            "subscription_schedule.released" => {
                Self::SubscriptionScheduleReleased(FromValueOpt::from_value(data)?)
            }
            "subscription_schedule.updated" => {
                Self::SubscriptionScheduleUpdated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_misc")]
            "tax.settings.updated" => Self::TaxSettingsUpdated(FromValueOpt::from_value(data)?),
            "tax_rate.created" => Self::TaxRateCreated(FromValueOpt::from_value(data)?),
            "tax_rate.updated" => Self::TaxRateUpdated(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_terminal")]
            "terminal.reader.action_failed" => {
                Self::TerminalReaderActionFailed(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_terminal")]
            "terminal.reader.action_succeeded" => {
                Self::TerminalReaderActionSucceeded(FromValueOpt::from_value(data)?)
            }
            "test_helpers.test_clock.advancing" => {
                Self::TestHelpersTestClockAdvancing(FromValueOpt::from_value(data)?)
            }
            "test_helpers.test_clock.created" => {
                Self::TestHelpersTestClockCreated(FromValueOpt::from_value(data)?)
            }
            "test_helpers.test_clock.deleted" => {
                Self::TestHelpersTestClockDeleted(FromValueOpt::from_value(data)?)
            }
            "test_helpers.test_clock.internal_failure" => {
                Self::TestHelpersTestClockInternalFailure(FromValueOpt::from_value(data)?)
            }
            "test_helpers.test_clock.ready" => {
                Self::TestHelpersTestClockReady(FromValueOpt::from_value(data)?)
            }
            "topup.canceled" => Self::TopupCanceled(FromValueOpt::from_value(data)?),
            "topup.created" => Self::TopupCreated(FromValueOpt::from_value(data)?),
            "topup.failed" => Self::TopupFailed(FromValueOpt::from_value(data)?),
            "topup.reversed" => Self::TopupReversed(FromValueOpt::from_value(data)?),
            "topup.succeeded" => Self::TopupSucceeded(FromValueOpt::from_value(data)?),
            "transfer.created" => Self::TransferCreated(FromValueOpt::from_value(data)?),
            "transfer.reversed" => Self::TransferReversed(FromValueOpt::from_value(data)?),
            "transfer.updated" => Self::TransferUpdated(FromValueOpt::from_value(data)?),
            #[cfg(feature = "stripe_treasury")]
            "treasury.credit_reversal.created" => {
                Self::TreasuryCreditReversalCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.credit_reversal.posted" => {
                Self::TreasuryCreditReversalPosted(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.debit_reversal.completed" => {
                Self::TreasuryDebitReversalCompleted(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.debit_reversal.created" => {
                Self::TreasuryDebitReversalCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.debit_reversal.initial_credit_granted" => {
                Self::TreasuryDebitReversalInitialCreditGranted(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.financial_account.closed" => {
                Self::TreasuryFinancialAccountClosed(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.financial_account.created" => {
                Self::TreasuryFinancialAccountCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.financial_account.features_status_updated" => {
                Self::TreasuryFinancialAccountFeaturesStatusUpdated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.inbound_transfer.canceled" => {
                Self::TreasuryInboundTransferCanceled(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.inbound_transfer.created" => {
                Self::TreasuryInboundTransferCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.inbound_transfer.failed" => {
                Self::TreasuryInboundTransferFailed(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.inbound_transfer.succeeded" => {
                Self::TreasuryInboundTransferSucceeded(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.canceled" => {
                Self::TreasuryOutboundPaymentCanceled(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.created" => {
                Self::TreasuryOutboundPaymentCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.expected_arrival_date_updated" => {
                Self::TreasuryOutboundPaymentExpectedArrivalDateUpdated(FromValueOpt::from_value(
                    data,
                )?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.failed" => {
                Self::TreasuryOutboundPaymentFailed(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.posted" => {
                Self::TreasuryOutboundPaymentPosted(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_payment.returned" => {
                Self::TreasuryOutboundPaymentReturned(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.canceled" => {
                Self::TreasuryOutboundTransferCanceled(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.created" => {
                Self::TreasuryOutboundTransferCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.expected_arrival_date_updated" => {
                Self::TreasuryOutboundTransferExpectedArrivalDateUpdated(FromValueOpt::from_value(
                    data,
                )?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.failed" => {
                Self::TreasuryOutboundTransferFailed(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.posted" => {
                Self::TreasuryOutboundTransferPosted(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.outbound_transfer.returned" => {
                Self::TreasuryOutboundTransferReturned(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.received_credit.created" => {
                Self::TreasuryReceivedCreditCreated(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.received_credit.failed" => {
                Self::TreasuryReceivedCreditFailed(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.received_credit.succeeded" => {
                Self::TreasuryReceivedCreditSucceeded(FromValueOpt::from_value(data)?)
            }
            #[cfg(feature = "stripe_treasury")]
            "treasury.received_debit.created" => {
                Self::TreasuryReceivedDebitCreated(FromValueOpt::from_value(data)?)
            }

            _ => Self::Unknown(data),
        })
    }
}
