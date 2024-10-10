use smart_default::SmartDefault;

macro_rules! def_id_serde_impls {
    ($struct_name:ident) => {
        impl serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                self.as_str().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let s: String = serde::Deserialize::deserialize(deserializer)?;
                s.parse::<Self>().map_err(::serde::de::Error::custom)
            }
        }
    };
    ($struct_name:ident, _) => {};
}

macro_rules! def_id {
    ($struct_name:ident: String) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
        pub struct $struct_name(smol_str::SmolStr);

        impl $struct_name {
            /// Extracts a string slice containing the entire id.
            #[inline(always)]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl PartialEq<str> for $struct_name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $struct_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for $struct_name {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other
            }
        }

        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $struct_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl crate::params::AsCursor for $struct_name {}

        impl std::ops::Deref for $struct_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $struct_name {
            type Err = ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($struct_name(s.into()))
            }
        }

        impl serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::ser::Serializer
            {
                self.as_str().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::de::Deserializer<'de>
            {
                let s: String = serde::Deserialize::deserialize(deserializer)?;
                s.parse::<Self>().map_err(::serde::de::Error::custom)
            }
        }
    };
    ($struct_name:ident, $prefix:literal $(| $alt_prefix:literal)* $(, { $generate_hint:tt })?) => {
        /// An id for the corresponding object type.
        ///
        /// This type _typically_ will not allocate and
        /// therefore is usually cheaply clonable.
        #[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
        pub struct $struct_name(smol_str::SmolStr);

        impl $struct_name {
            /// The prefix of the id type (e.g. `cus_` for a `CustomerId`).
            #[inline(always)]
            #[deprecated(note = "Please use prefixes or is_valid_prefix")]
            pub fn prefix() -> &'static str {
                $prefix
            }

            /// The valid prefixes of the id type (e.g. [`ch_`, `py_`\ for a `ChargeId`).
            #[inline(always)]
            pub fn prefixes() -> &'static [&'static str] {
                &[$prefix$(, $alt_prefix)*]
            }

            /// Extracts a string slice containing the entire id.
            #[inline(always)]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            /// Check is provided prefix would be a valid prefix for id's of this type
            pub fn is_valid_prefix(prefix: &str) -> bool {
                prefix == $prefix $( || prefix == $alt_prefix )*
            }
        }

        impl PartialEq<str> for $struct_name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $struct_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for $struct_name {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other
            }
        }

        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $struct_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl crate::params::AsCursor for $struct_name {}

        impl std::ops::Deref for $struct_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $struct_name {
            type Err = ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if !s.starts_with($prefix) $(
                    && !s.starts_with($alt_prefix)
                )* {
                    // N.B. For debugging
                    eprintln!("bad id is: {} (expected: {:?}) for {}", s, $prefix, stringify!($struct_name));

                    Err(ParseIdError {
                        typename: stringify!($struct_name),
                        expected: stringify!(id to start with $prefix $(or $alt_prefix)*),
                    })
                } else {
                    Ok($struct_name(s.into()))
                }
            }
        }

        def_id_serde_impls!($struct_name $(, $generate_hint )*);
    };
    (#[optional] enum $enum_name:ident { $( $variant_name:ident($($variant_type:tt)*) ),* $(,)* }) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum $enum_name {
            None,
            $( $variant_name($($variant_type)*), )*
        }

        impl $enum_name {
            pub fn as_str(&self) -> &str {
                match *self {
                    $enum_name::None => "",
                    $( $enum_name::$variant_name(ref id) => id.as_str(), )*
                }
            }
        }

        impl PartialEq<str> for $enum_name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $enum_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for $enum_name {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other
            }
        }

        impl AsRef<str> for $enum_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl crate::params::AsCursor for $enum_name {}

        impl std::ops::Deref for $enum_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $enum_name::None => Ok(()),
                    $( $enum_name::$variant_name(ref id) => id.fmt(f), )*
                }
            }
        }

        impl std::default::Default for $enum_name {
            fn default() -> Self {
                $enum_name::None
            }
        }

        impl std::str::FromStr for $enum_name {
            type Err = ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let prefix = s.find('_')
                    .map(|i| &s[0..=i])
                    .ok_or_else(|| ParseIdError {
                        typename: stringify!($enum_name),
                        expected: "id to start with a prefix (as in 'prefix_')"
                    })?;

                match prefix {
                    $(_ if $($variant_type)*::is_valid_prefix(prefix) => {
                        Ok($enum_name::$variant_name(s.parse()?))
                    })*
                    _ => {
                        Err(ParseIdError {
                            typename: stringify!($enum_name),
                            expected: "unknown id prefix",
                        })
                    }
                }
            }
        }

        impl serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::ser::Serializer
            {
                self.as_str().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::de::Deserializer<'de>
            {
                let s: String = serde::Deserialize::deserialize(deserializer)?;
                s.parse::<Self>().map_err(::serde::de::Error::custom)
            }
        }

        $(
            impl From<$($variant_type)*> for $enum_name {
                fn from(id: $($variant_type)*) -> Self {
                    $enum_name::$variant_name(id)
                }
            }
        )*
    };
    (enum $enum_name:ident { $( $(#[$test:meta])? $variant_name:ident($($variant_type:tt)*) ),+ $(,)? }) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[derive(SmartDefault)]
        pub enum $enum_name {
            $( $(#[$test])* $variant_name($($variant_type)*), )*
        }

        impl $enum_name {
            pub fn as_str(&self) -> &str {
                match *self {
                    $( $enum_name::$variant_name(ref id) => id.as_str(), )*
                }
            }
        }

        impl PartialEq<str> for $enum_name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $enum_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for $enum_name {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other
            }
        }

        impl AsRef<str> for $enum_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl crate::params::AsCursor for $enum_name {}

        impl std::ops::Deref for $enum_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $( $enum_name::$variant_name(ref id) => id.fmt(f), )*
                }
            }
        }

        impl std::str::FromStr for $enum_name {
            type Err = ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let prefix = s.find('_')
                    .map(|i| &s[0..=i])
                    .ok_or_else(|| ParseIdError {
                        typename: stringify!($enum_name),
                        expected: "id to start with a prefix (as in 'prefix_')"
                    })?;

                match prefix {
                    $(_ if $($variant_type)*::is_valid_prefix(prefix) => {
                        Ok($enum_name::$variant_name(s.parse()?))
                    })*
                    _ => {
                        Err(ParseIdError {
                            typename: stringify!($enum_name),
                            expected: "unknown id prefix",
                        })
                    }
                }
            }
        }

        impl serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::ser::Serializer
            {
                self.as_str().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::de::Deserializer<'de>
            {
                let s: String = serde::Deserialize::deserialize(deserializer)?;
                s.parse::<Self>().map_err(::serde::de::Error::custom)
            }
        }

        $(
            impl From<$($variant_type)*> for $enum_name {
                fn from(id: $($variant_type)*) -> Self {
                    $enum_name::$variant_name(id)
                }
            }
        )*
    };
}

#[derive(Clone, Debug)]
pub struct ParseIdError {
    typename: &'static str,
    expected: &'static str,
}

impl std::fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid `{}`, expected {}", self.typename, self.expected)
    }
}

impl std::error::Error for ParseIdError {
    fn description(&self) -> &str {
        "error parsing an id"
    }
}

def_id!(AccountId, "acct_");
def_id!(AlipayAccountId, "aliacc_");
def_id!(ApplicationFeeId, "fee_");
def_id!(ApplicationId, "ca_");
def_id!(ApplicationFeeRefundId, "fr_");
def_id!(BalanceTransactionId, "txn_");
def_id!(BankAccountId, "ba_" | "card_");
def_id!(BillingPortalSessionId, "bps_");
def_id!(BillingPortalConfigurationId, "bpc_");
def_id!(BankTokenId, "btok_");
def_id!(
    #[optional]
    enum BalanceTransactionSourceId {
        ApplicationFee(ApplicationFeeId),
        Charge(ChargeId),
        Dispute(DisputeId),
        ApplicationFeeRefund(ApplicationFeeRefundId),
        IssuingAuthorization(IssuingAuthorizationId),
        IssuingDispute(IssuingDisputeId),
        IssuingTransaction(IssuingTransactionId),
        Payout(PayoutId),
        Refund(RefundId),
        Topup(TopupId),
        Transfer(TransferId),
        TransferReversal(TransferReversalId),
    }
);
def_id!(CardId, "card_");
def_id!(CardTokenId, "tok_");
def_id!(ChargeId, "ch_" | "py_"); // TODO: Understand (and then document) why "py_" is a valid charge id
def_id!(CheckoutSessionId, "cs_");
def_id!(CheckoutSessionItemId, "li_");
def_id!(ConnectCollectionTransferId, "connct_");
def_id!(ConnectTokenId, "ct_");
def_id!(CouponId: String); // N.B. A coupon id can be user-provided so can be any arbitrary string
def_id!(CreditNoteId, "cn_");
def_id!(CreditNoteLineItemId, "cnli_");
def_id!(CustomerBalanceTransactionId, "cbtxn_");
def_id!(CustomerId, "cus_");
def_id!(DiscountId, "di_");
def_id!(DisputeId, "dp_" | "du_" | "pdp_");
def_id!(EphemeralKeyId, "ephkey_");
def_id!(EventId, "evt_");
def_id!(FileId, "file_");
def_id!(FileLinkId, "link_");
def_id!(InvoiceId, "in_", { _ });
def_id!(InvoiceItemId, "ii_");
def_id!(InvoiceLineItemIdWebhook, "il_");

def_id!(
    enum InvoiceLineItemId {
        #[default]
        Item(InvoiceItemId),
        Subscription(SubscriptionLineId),
        InvoiceLineItemIdWebhook(InvoiceLineItemIdWebhook),
    }
);
def_id!(IssuingAuthorizationId, "iauth_");
def_id!(IssuingCardId, "ic_");
def_id!(IssuingCardholderId, "ich_");
def_id!(IssuingDisputeId, "idp_");
def_id!(IssuingTransactionId, "ipi_");
def_id!(IssuingTokenId: String);
def_id!(OrderId, "or_");
def_id!(OrderReturnId, "orret_");
def_id!(MandateId, "mandate_");
def_id!(PaymentMethodConfigurationId: String);
def_id!(PaymentIntentId, "pi_");
def_id!(PaymentLinkId, "plink_");
def_id!(PaymentMethodId, "pm_" | "card_" | "src_" | "ba_");
def_id!(
    enum PaymentSourceId {
        #[default]
        Account(AccountId),
        AlipayAccount(AlipayAccountId),
        BankAccount(BankAccountId),
        Card(CardId),
        Source(SourceId),
    }
);
def_id!(PayoutId, "po_");
def_id!(
    enum PayoutDestinationId {
        #[default]
        BankAccount(BankAccountId),
        Card(CardId),
    }
);
def_id!(PersonId, "person_");
def_id!(PlanId: String); // N.B. A plan id can be user-provided so can be any arbitrary string
def_id!(PlatformTaxFeeId, "ptf");
def_id!(PriceId: String); // N.B. A price id can be user-provided so can be any arbitrary string
def_id!(ProductId: String); // N.B. A product id can be user-provided so can be any arbitrary string
def_id!(PromotionCodeId, "promo_");
def_id!(QuoteId, "qt_");
def_id!(RecipientId: String); // FIXME: This doesn't seem to be documented yet
def_id!(RefundId, "re_" | "pyr_");
def_id!(ReserveTransactionId, "rtx_");
def_id!(ReviewId, "prv_");
def_id!(ScheduledQueryRunId, "sqr_");
def_id!(SetupAttemptId, "setatt_");
def_id!(SetupIntentId, "seti_");
def_id!(SkuId, "sku_");
def_id!(ShippingRateId, "shr_");
def_id!(SourceId, "src_");
def_id!(SubscriptionId, "sub_");
def_id!(SubscriptionItemId, "si_");
def_id!(SubscriptionLineId, "sli_");
def_id!(SubscriptionScheduleId, "sub_sched_");
def_id!(TaxIdId, "txi_" | "atxi_");
def_id!(TaxCalculationId: String);
def_id!(TaxCalculationLineItemId: String);
def_id!(TaxCodeId, "txcd_");
def_id!(TaxDeductedAtSourceId, "itds");
def_id!(TaxRateId, "txr_");
def_id!(TerminalConfigurationId, "tmc_");
def_id!(TerminalLocationId, "tml_");
def_id!(TerminalReaderId, "tmr_");
def_id!(TestHelpersTestClockId, "clock_");
def_id!(
    enum TokenId {
        #[default]
        Card(CardTokenId),
        Bank(BankTokenId),
        Connect(ConnectTokenId),
    }
);
def_id!(TopupId, "tu_");
def_id!(TransferId, "tr_");
def_id!(TransferReversalId, "trr_");
def_id!(UsageRecordId, "mbur_");
def_id!(UsageRecordSummaryId, "urs_" | "sis_");
def_id!(WebhookEndpointId, "we_");

impl InvoiceId {
    pub(crate) fn none() -> Self {
        Self("".into())
    }

    /// An InvoiceId may have a `None` representation when
    /// received from Stripe if the Invoice is an upcoming invoice.
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }
}
impl serde::Serialize for InvoiceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        if self.0.is_empty() {
            let val: Option<&str> = None;
            val.serialize(serializer)
        } else {
            self.as_str().serialize(serializer)
        }
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s: Option<String> = serde::Deserialize::deserialize(deserializer)?;
        match s {
            None => Ok(InvoiceId::none()),
            Some(s) => {
                if s.is_empty() {
                    Ok(InvoiceId::none())
                } else {
                    s.parse::<Self>().map_err(::serde::de::Error::custom)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display};
    use std::str::FromStr;

    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use super::*;

    fn assert_ser_de_roundtrip<T>(id: &str)
    where
        T: DeserializeOwned + Serialize + FromStr + Display + Debug,
        <T as FromStr>::Err: Debug,
    {
        let parsed_id = T::from_str(id).expect("Could not parse id");
        let ser = serde_json::to_string(&parsed_id).expect("Could not serialize id");
        let deser: T = serde_json::from_str(&ser).expect("Could not deserialize id");
        assert_eq!(deser.to_string(), id.to_string());
    }

    fn assert_deser_err<T: DeserializeOwned + Debug>(id: &str) {
        let json_str = format!(r#""{}""#, id);
        let deser: Result<T, _> = serde_json::from_str(&json_str);
        assert!(deser.is_err(), "Expected error, got {:?}", deser);
    }

    #[test]
    fn test_empty_invoice_id_default() {
        #[derive(Deserialize)]
        struct WithInvoiceId {
            id: InvoiceId,
        }

        for body in [json!({"id": ""}), json!({})] {
            let deser: WithInvoiceId = serde_json::from_value(body).expect("Could not deser");
            assert_eq!(deser.id, InvoiceId::none());
        }
    }

    #[test]
    fn test_ser_de_roundtrip() {
        // InvoiceId special cased
        for id in ["in_12345", "in_"] {
            assert_ser_de_roundtrip::<InvoiceId>(id);
        }

        // Single prefix
        assert_ser_de_roundtrip::<PriceId>("price_abc");

        // Case where multiple possible prefixes
        for id in ["re_bcd", "pyr_123"] {
            assert_ser_de_roundtrip::<RefundId>(id);
        }

        // Case where id can be anything
        for id in ["anything", ""] {
            assert_ser_de_roundtrip::<ProductId>(id);
        }

        // Case where enum id
        for id in ["tok_123", "btok_456"] {
            assert_ser_de_roundtrip::<TokenId>(id);
        }
    }

    #[test]
    fn test_deser_err() {
        // InvoiceId special cased
        assert_deser_err::<InvoiceId>("in");

        // Single prefix
        for id in ["sub", ""] {
            assert_deser_err::<SubscriptionId>(id);
        }

        // Case where multiple possible prefixes
        for id in ["abc_bcd", "pyr_123"] {
            assert_deser_err::<PaymentMethodId>(id);
        }

        // Case where enum id
        for id in ["tok_123", "btok_456"] {
            assert_deser_err::<PaymentSourceId>(id);
        }
    }

    #[test]
    fn test_parse_customer() {
        assert!("cus_123".parse::<CustomerId>().is_ok());
        let bad_parse = "zzz_123".parse::<CustomerId>();
        assert!(bad_parse.is_err());
        if let Err(err) = bad_parse {
            assert_eq!(
                format!("{}", err),
                "invalid `CustomerId`, expected id to start with \"cus_\""
            );
        }
    }

    #[test]
    fn test_parse_charge() {
        assert!("ch_123".parse::<ChargeId>().is_ok());
        assert!("py_123".parse::<ChargeId>().is_ok());
        let bad_parse = "zz_123".parse::<ChargeId>();
        assert!(bad_parse.is_err());
        if let Err(err) = bad_parse {
            assert_eq!(
                format!("{}", err),
                "invalid `ChargeId`, expected id to start with \"ch_\" or \"py_\""
            );
        }
    }
}
