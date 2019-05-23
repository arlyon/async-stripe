macro_rules! def_id {
    ($struct_name:ident: $COMMENT:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
    ($struct_name:ident, $prefix:expr) => {
        /// An id for the corresponding object type.
        ///
        /// This type _typically_ will not allocate and
        /// therefore is usually cheaply clonable.
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub struct $struct_name(smol_str::SmolStr);

        impl $struct_name {
            /// The prefix of the id type (e.g. `cus_` for a `CustomerId`).
            #[inline(always)]
            pub fn prefix() -> &'static str {
                $prefix
            }

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
                if !s.starts_with($prefix) {

                    // N.B. For debugging
                    eprintln!("bad id is: {}", s);

                    Err(ParseIdError {
                        typename: stringify!($struct_name),
                        expected: stringify!(id to start with $prefix),
                    })
                } else {
                    Ok($struct_name(s.into()))
                }
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

    ($enum_name:ident { $( $variant_name:ident($($variant_type:tt)*) ),* $(,)* }) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum $enum_name {$(
            $variant_name($($variant_type)*),
        )*}

        impl $enum_name {
            pub fn as_str(&self) -> &str {
                match *self {$(
                    $enum_name::$variant_name(ref id) => id.as_str(),
                )*}
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
                match *self {$(
                    $enum_name::$variant_name(ref id) => id.fmt(f),
                )*}
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
                    $(_ if prefix == $($variant_type)*::prefix() => {
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
def_id!(ApplicationFeeRefundId, "fr_");
def_id!(BalanceTransactionId, "txn_");
def_id!(BankAccountId, "ba_");
def_id!(BankTokenId, "btok_");
def_id!(CardId, "card_");
def_id!(CardTokenId, "tok_");
def_id!(ChargeId, "ch_");
def_id!(CouponId: USER_PROVIDED); // N.B. A coupon id can be user-provided so can be any arbitrary string
def_id!(CustomerId, "cus_");
def_id!(DisputeId, "dp_");
def_id!(FileId, "file_");
def_id!(FileLinkId, "link_");
def_id!(InvoiceId, "in_");
def_id!(InvoiceItemId, "ii_");
def_id!(InvoiceLineItemId, "ii_"); // N.B. yes this is the same as `InvoiceItemId`
def_id!(IssuingAuthorizationId, "iauth_");
def_id!(IssuingCardId, "ic_");
def_id!(IssuingCardholderId, "ich_");
def_id!(IssuingDisputeId, "idp_");
def_id!(IssuingTransactionId, "ipi_");
def_id!(OrderId, "or_");
def_id!(OrderReturnId, "orret_");
def_id!(PaymentIntentId, "pi_");
def_id!(PaymentMethodId, "pm");
def_id!(PaymentSourceId {
    Account(AccountId),
    AlipayAccount(AlipayAccountId),
    BankAccount(BankAccountId),
    Card(CardId),
    Source(SourceId),
});
def_id!(PayoutId, "po_");
def_id!(PlanId: USER_PROVIDED); // N.B. A plan id can be user-provided so can be any arbitrary string
def_id!(ProductId, "prod_");
def_id!(RecipientId: UNDOCUMENTED); // FIXME: This doesn't seem to be documented yet
def_id!(RefundId, "re_");
def_id!(ReviewId, "prv_");
def_id!(ScheduledQueryRunId, "sqr_");
def_id!(SkuId, "sku_");
def_id!(SourceId, "src_");
def_id!(SubscriptionId, "sub_");
def_id!(SubscriptionItemId, "si_");
def_id!(TaxRateId, "txr_");
def_id!(TokenId {
    Card(CardTokenId),
    Bank(BankTokenId),
});
def_id!(TopupId, "tu_");
def_id!(TransferId, "tr_");
def_id!(TransferReversalId, "trr_");
