#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceOwner {
    /// Owner's address.
    pub address: Option<stripe_shared::Address>,
    /// Owner's email address.
    pub email: Option<String>,
    /// Owner's full name.
    pub name: Option<String>,
    /// Owner's phone number (including extension).
    pub phone: Option<String>,
    /// Verified owner's address.
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_address: Option<stripe_shared::Address>,
    /// Verified owner's email address.
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_email: Option<String>,
    /// Verified owner's full name.
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
    /// Verified owner's phone number (including extension).
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_phone: Option<String>,
}
#[doc(hidden)]
pub struct SourceOwnerBuilder {
    address: Option<Option<stripe_shared::Address>>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
    verified_address: Option<Option<stripe_shared::Address>>,
    verified_email: Option<Option<String>>,
    verified_name: Option<Option<String>>,
    verified_phone: Option<Option<String>>,
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

    impl Deserialize for SourceOwner {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceOwner>,
        builder: SourceOwnerBuilder,
    }

    impl Visitor for Place<SourceOwner> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceOwnerBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceOwnerBuilder {
        type Out = SourceOwner;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "email" => Deserialize::begin(&mut self.email),
                "name" => Deserialize::begin(&mut self.name),
                "phone" => Deserialize::begin(&mut self.phone),
                "verified_address" => Deserialize::begin(&mut self.verified_address),
                "verified_email" => Deserialize::begin(&mut self.verified_email),
                "verified_name" => Deserialize::begin(&mut self.verified_name),
                "verified_phone" => Deserialize::begin(&mut self.verified_phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                email: Deserialize::default(),
                name: Deserialize::default(),
                phone: Deserialize::default(),
                verified_address: Deserialize::default(),
                verified_email: Deserialize::default(),
                verified_name: Deserialize::default(),
                verified_phone: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address),
                Some(email),
                Some(name),
                Some(phone),
                Some(verified_address),
                Some(verified_email),
                Some(verified_name),
                Some(verified_phone),
            ) = (
                self.address.take(),
                self.email.take(),
                self.name.take(),
                self.phone.take(),
                self.verified_address.take(),
                self.verified_email.take(),
                self.verified_name.take(),
                self.verified_phone.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                address,
                email,
                name,
                phone,
                verified_address,
                verified_email,
                verified_name,
                verified_phone,
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

    impl ObjectDeser for SourceOwner {
        type Builder = SourceOwnerBuilder;
    }

    impl FromValueOpt for SourceOwner {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceOwnerBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    "verified_address" => b.verified_address = FromValueOpt::from_value(v),
                    "verified_email" => b.verified_email = FromValueOpt::from_value(v),
                    "verified_name" => b.verified_name = FromValueOpt::from_value(v),
                    "verified_phone" => b.verified_phone = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
