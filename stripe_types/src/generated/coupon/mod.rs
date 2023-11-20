/// A coupon contains information about a percent-off or amount-off discount you
/// might want to apply to a customer.
///
/// Coupons may be applied to [subscriptions](https://stripe.com/docs/api#subscriptions), [invoices](https://stripe.com/docs/api#invoices), [checkout sessions](https://stripe.com/docs/api/checkout/sessions), [quotes](https://stripe.com/docs/api#quotes), and more.
/// Coupons do not work with conventional one-off [charges](https://stripe.com/docs/api#create_charge) or [payment intents](https://stripe.com/docs/api/payment_intents).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Coupon {
    /// Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<stripe_types::CouponAppliesTo>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If `amount_off` has been set, the three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the amount to take off.
    pub currency: Option<stripe_types::Currency>,
    /// Coupons defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        std::collections::HashMap<stripe_types::Currency, stripe_types::CouponCurrencyOption>,
    >,
    /// One of `forever`, `once`, and `repeating`.
    ///
    /// Describes how long a customer who applies this coupon will get the discount.
    pub duration: CouponDuration,
    /// If `duration` is `repeating`, the number of months the coupon applies.
    ///
    /// Null if coupon `duration` is `forever` or `once`.
    pub duration_in_months: Option<i64>,
    /// Unique identifier for the object.
    pub id: stripe_types::coupon::CouponId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Maximum number of times this coupon can be redeemed, in total, across all customers, before it is no longer valid.
    pub max_redemptions: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Name of the coupon displayed to customers on for instance invoices or receipts.
    pub name: Option<String>,
    /// Percent that will be taken off the subtotal of any invoices for this customer for the duration of the coupon.
    ///
    /// For example, a coupon with percent_off of 50 will make a $ (or local equivalent)100 invoice $ (or local equivalent)50 instead.
    pub percent_off: Option<f64>,
    /// Date after which the coupon can no longer be redeemed.
    pub redeem_by: Option<stripe_types::Timestamp>,
    /// Number of times this coupon has been applied to a customer.
    pub times_redeemed: i64,
    /// Taking account of the above properties, whether this coupon can still be applied to a customer.
    pub valid: bool,
}
/// One of `forever`, `once`, and `repeating`.
///
/// Describes how long a customer who applies this coupon will get the discount.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CouponDuration {
    Forever,
    Once,
    Repeating,
}

impl CouponDuration {
    pub fn as_str(self) -> &'static str {
        use CouponDuration::*;
        match self {
            Forever => "forever",
            Once => "once",
            Repeating => "repeating",
        }
    }
}

impl std::str::FromStr for CouponDuration {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CouponDuration::*;
        match s {
            "forever" => Ok(Forever),
            "once" => Ok(Once),
            "repeating" => Ok(Repeating),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CouponDuration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CouponDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CouponDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CouponDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CouponDuration {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CouponDuration"))
    }
}
impl stripe_types::Object for Coupon {
    type Id = stripe_types::coupon::CouponId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(CouponId);
