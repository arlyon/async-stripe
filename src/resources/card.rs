#[derive(Serialize)]
pub struct CardParams {
    pub object: &'static str, // must be "card"
    pub exp_month: String,    // eg. "12"
    pub exp_year: String,     // eg. "17" or 2017"

    pub number: String,       // card number
    pub cvc: Option<String>,  // card security code
    pub name: Option<String>, // cardholder's full name
}

#[derive(Debug, Deserialize)]
pub struct Card {
    pub id: String,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<String>, // (pass, fail, unavailable, unchecked)
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<String>, // (pass, fail, unavailable, unchecked)
    pub brand: String, // (Visa, American Express, MasterCard, Discover, JCB, Diners Club, or Unknown)
    pub country: String, // eg. "US"
    pub customer: Option<String>,
    pub cvc_check: Option<String>, // (pass, fail, unavailable, unchecked)
    pub exp_month: u32,
    pub exp_year: u32,
    pub fingerprint: String,
    pub funding: String, // (credit, debit, prepaid, unknown)
}
