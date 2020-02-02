
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains the type of a Telegram Passport element
pub trait TDPassportElementType: Debug + RObject {}

/// Contains the type of a Telegram Passport element
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PassportElementType {
  #[doc(hidden)] _Default(()),
  /// A Telegram Passport element containing the user's personal details
  PersonalDetails(PassportElementTypePersonalDetails),
  /// A Telegram Passport element containing the user's passport
  Passport(PassportElementTypePassport),
  /// A Telegram Passport element containing the user's driver license
  DriverLicense(PassportElementTypeDriverLicense),
  /// A Telegram Passport element containing the user's identity card
  IdentityCard(PassportElementTypeIdentityCard),
  /// A Telegram Passport element containing the user's internal passport
  InternalPassport(PassportElementTypeInternalPassport),
  /// A Telegram Passport element containing the user's address
  Address(PassportElementTypeAddress),
  /// A Telegram Passport element containing the user's utility bill
  UtilityBill(PassportElementTypeUtilityBill),
  /// A Telegram Passport element containing the user's bank statement
  BankStatement(PassportElementTypeBankStatement),
  /// A Telegram Passport element containing the user's rental agreement
  RentalAgreement(PassportElementTypeRentalAgreement),
  /// A Telegram Passport element containing the registration page of the user's passport
  PassportRegistration(PassportElementTypePassportRegistration),
  /// A Telegram Passport element containing the user's temporary registration
  TemporaryRegistration(PassportElementTypeTemporaryRegistration),
  /// A Telegram Passport element containing the user's phone number
  PhoneNumber(PassportElementTypePhoneNumber),
  /// A Telegram Passport element containing the user's email address
  EmailAddress(PassportElementTypeEmailAddress),

}

impl Default for PassportElementType {
  fn default() -> Self { PassportElementType::_Default(()) }
}

impl<'de> Deserialize<'de> for PassportElementType {
  fn deserialize<D>(deserializer: D) -> Result<PassportElementType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PassportElementType,
      (passportElementTypePersonalDetails, PersonalDetails);
      (passportElementTypePassport, Passport);
      (passportElementTypeDriverLicense, DriverLicense);
      (passportElementTypeIdentityCard, IdentityCard);
      (passportElementTypeInternalPassport, InternalPassport);
      (passportElementTypeAddress, Address);
      (passportElementTypeUtilityBill, UtilityBill);
      (passportElementTypeBankStatement, BankStatement);
      (passportElementTypeRentalAgreement, RentalAgreement);
      (passportElementTypePassportRegistration, PassportRegistration);
      (passportElementTypeTemporaryRegistration, TemporaryRegistration);
      (passportElementTypePhoneNumber, PhoneNumber);
      (passportElementTypeEmailAddress, EmailAddress);

    )(deserializer)
  }
}

impl RObject for PassportElementType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      PassportElementType::PersonalDetails(t) => t.td_name(),
      PassportElementType::Passport(t) => t.td_name(),
      PassportElementType::DriverLicense(t) => t.td_name(),
      PassportElementType::IdentityCard(t) => t.td_name(),
      PassportElementType::InternalPassport(t) => t.td_name(),
      PassportElementType::Address(t) => t.td_name(),
      PassportElementType::UtilityBill(t) => t.td_name(),
      PassportElementType::BankStatement(t) => t.td_name(),
      PassportElementType::RentalAgreement(t) => t.td_name(),
      PassportElementType::PassportRegistration(t) => t.td_name(),
      PassportElementType::TemporaryRegistration(t) => t.td_name(),
      PassportElementType::PhoneNumber(t) => t.td_name(),
      PassportElementType::EmailAddress(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl PassportElementType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let PassportElementType::_Default(_) = self { true } else { false } }

  pub fn is_personal_details(&self) -> bool { if let PassportElementType::PersonalDetails(_) = self { true } else { false } }
  pub fn is_passport(&self) -> bool { if let PassportElementType::Passport(_) = self { true } else { false } }
  pub fn is_driver_license(&self) -> bool { if let PassportElementType::DriverLicense(_) = self { true } else { false } }
  pub fn is_identity_card(&self) -> bool { if let PassportElementType::IdentityCard(_) = self { true } else { false } }
  pub fn is_internal_passport(&self) -> bool { if let PassportElementType::InternalPassport(_) = self { true } else { false } }
  pub fn is_address(&self) -> bool { if let PassportElementType::Address(_) = self { true } else { false } }
  pub fn is_utility_bill(&self) -> bool { if let PassportElementType::UtilityBill(_) = self { true } else { false } }
  pub fn is_bank_statement(&self) -> bool { if let PassportElementType::BankStatement(_) = self { true } else { false } }
  pub fn is_rental_agreement(&self) -> bool { if let PassportElementType::RentalAgreement(_) = self { true } else { false } }
  pub fn is_passport_registration(&self) -> bool { if let PassportElementType::PassportRegistration(_) = self { true } else { false } }
  pub fn is_temporary_registration(&self) -> bool { if let PassportElementType::TemporaryRegistration(_) = self { true } else { false } }
  pub fn is_phone_number(&self) -> bool { if let PassportElementType::PhoneNumber(_) = self { true } else { false } }
  pub fn is_email_address(&self) -> bool { if let PassportElementType::EmailAddress(_) = self { true } else { false } }

  pub fn on_personal_details<F: FnOnce(&PassportElementTypePersonalDetails)>(&self, fnc: F) -> &Self { if let PassportElementType::PersonalDetails(t) = self { fnc(t) }; self }
  pub fn on_passport<F: FnOnce(&PassportElementTypePassport)>(&self, fnc: F) -> &Self { if let PassportElementType::Passport(t) = self { fnc(t) }; self }
  pub fn on_driver_license<F: FnOnce(&PassportElementTypeDriverLicense)>(&self, fnc: F) -> &Self { if let PassportElementType::DriverLicense(t) = self { fnc(t) }; self }
  pub fn on_identity_card<F: FnOnce(&PassportElementTypeIdentityCard)>(&self, fnc: F) -> &Self { if let PassportElementType::IdentityCard(t) = self { fnc(t) }; self }
  pub fn on_internal_passport<F: FnOnce(&PassportElementTypeInternalPassport)>(&self, fnc: F) -> &Self { if let PassportElementType::InternalPassport(t) = self { fnc(t) }; self }
  pub fn on_address<F: FnOnce(&PassportElementTypeAddress)>(&self, fnc: F) -> &Self { if let PassportElementType::Address(t) = self { fnc(t) }; self }
  pub fn on_utility_bill<F: FnOnce(&PassportElementTypeUtilityBill)>(&self, fnc: F) -> &Self { if let PassportElementType::UtilityBill(t) = self { fnc(t) }; self }
  pub fn on_bank_statement<F: FnOnce(&PassportElementTypeBankStatement)>(&self, fnc: F) -> &Self { if let PassportElementType::BankStatement(t) = self { fnc(t) }; self }
  pub fn on_rental_agreement<F: FnOnce(&PassportElementTypeRentalAgreement)>(&self, fnc: F) -> &Self { if let PassportElementType::RentalAgreement(t) = self { fnc(t) }; self }
  pub fn on_passport_registration<F: FnOnce(&PassportElementTypePassportRegistration)>(&self, fnc: F) -> &Self { if let PassportElementType::PassportRegistration(t) = self { fnc(t) }; self }
  pub fn on_temporary_registration<F: FnOnce(&PassportElementTypeTemporaryRegistration)>(&self, fnc: F) -> &Self { if let PassportElementType::TemporaryRegistration(t) = self { fnc(t) }; self }
  pub fn on_phone_number<F: FnOnce(&PassportElementTypePhoneNumber)>(&self, fnc: F) -> &Self { if let PassportElementType::PhoneNumber(t) = self { fnc(t) }; self }
  pub fn on_email_address<F: FnOnce(&PassportElementTypeEmailAddress)>(&self, fnc: F) -> &Self { if let PassportElementType::EmailAddress(t) = self { fnc(t) }; self }

  pub fn as_personal_details(&self) -> Option<&PassportElementTypePersonalDetails> { if let PassportElementType::PersonalDetails(t) = self { return Some(t) } None }
  pub fn as_passport(&self) -> Option<&PassportElementTypePassport> { if let PassportElementType::Passport(t) = self { return Some(t) } None }
  pub fn as_driver_license(&self) -> Option<&PassportElementTypeDriverLicense> { if let PassportElementType::DriverLicense(t) = self { return Some(t) } None }
  pub fn as_identity_card(&self) -> Option<&PassportElementTypeIdentityCard> { if let PassportElementType::IdentityCard(t) = self { return Some(t) } None }
  pub fn as_internal_passport(&self) -> Option<&PassportElementTypeInternalPassport> { if let PassportElementType::InternalPassport(t) = self { return Some(t) } None }
  pub fn as_address(&self) -> Option<&PassportElementTypeAddress> { if let PassportElementType::Address(t) = self { return Some(t) } None }
  pub fn as_utility_bill(&self) -> Option<&PassportElementTypeUtilityBill> { if let PassportElementType::UtilityBill(t) = self { return Some(t) } None }
  pub fn as_bank_statement(&self) -> Option<&PassportElementTypeBankStatement> { if let PassportElementType::BankStatement(t) = self { return Some(t) } None }
  pub fn as_rental_agreement(&self) -> Option<&PassportElementTypeRentalAgreement> { if let PassportElementType::RentalAgreement(t) = self { return Some(t) } None }
  pub fn as_passport_registration(&self) -> Option<&PassportElementTypePassportRegistration> { if let PassportElementType::PassportRegistration(t) = self { return Some(t) } None }
  pub fn as_temporary_registration(&self) -> Option<&PassportElementTypeTemporaryRegistration> { if let PassportElementType::TemporaryRegistration(t) = self { return Some(t) } None }
  pub fn as_phone_number(&self) -> Option<&PassportElementTypePhoneNumber> { if let PassportElementType::PhoneNumber(t) = self { return Some(t) } None }
  pub fn as_email_address(&self) -> Option<&PassportElementTypeEmailAddress> { if let PassportElementType::EmailAddress(t) = self { return Some(t) } None }



  pub fn personal_details<T: AsRef<PassportElementTypePersonalDetails>>(t: T) -> Self { PassportElementType::PersonalDetails(t.as_ref().clone()) }

  pub fn passport<T: AsRef<PassportElementTypePassport>>(t: T) -> Self { PassportElementType::Passport(t.as_ref().clone()) }

  pub fn driver_license<T: AsRef<PassportElementTypeDriverLicense>>(t: T) -> Self { PassportElementType::DriverLicense(t.as_ref().clone()) }

  pub fn identity_card<T: AsRef<PassportElementTypeIdentityCard>>(t: T) -> Self { PassportElementType::IdentityCard(t.as_ref().clone()) }

  pub fn internal_passport<T: AsRef<PassportElementTypeInternalPassport>>(t: T) -> Self { PassportElementType::InternalPassport(t.as_ref().clone()) }

  pub fn address<T: AsRef<PassportElementTypeAddress>>(t: T) -> Self { PassportElementType::Address(t.as_ref().clone()) }

  pub fn utility_bill<T: AsRef<PassportElementTypeUtilityBill>>(t: T) -> Self { PassportElementType::UtilityBill(t.as_ref().clone()) }

  pub fn bank_statement<T: AsRef<PassportElementTypeBankStatement>>(t: T) -> Self { PassportElementType::BankStatement(t.as_ref().clone()) }

  pub fn rental_agreement<T: AsRef<PassportElementTypeRentalAgreement>>(t: T) -> Self { PassportElementType::RentalAgreement(t.as_ref().clone()) }

  pub fn passport_registration<T: AsRef<PassportElementTypePassportRegistration>>(t: T) -> Self { PassportElementType::PassportRegistration(t.as_ref().clone()) }

  pub fn temporary_registration<T: AsRef<PassportElementTypeTemporaryRegistration>>(t: T) -> Self { PassportElementType::TemporaryRegistration(t.as_ref().clone()) }

  pub fn phone_number<T: AsRef<PassportElementTypePhoneNumber>>(t: T) -> Self { PassportElementType::PhoneNumber(t.as_ref().clone()) }

  pub fn email_address<T: AsRef<PassportElementTypeEmailAddress>>(t: T) -> Self { PassportElementType::EmailAddress(t.as_ref().clone()) }

}

impl AsRef<PassportElementType> for PassportElementType {
  fn as_ref(&self) -> &PassportElementType { self }
}







/// A Telegram Passport element containing the user's personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePersonalDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypePersonalDetails {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypePersonalDetails" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypePersonalDetails {}



impl PassportElementTypePersonalDetails {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypePersonalDetailsBuilder {
    let mut inner = PassportElementTypePersonalDetails::default();
    inner.td_name = "passportElementTypePersonalDetails".to_string();
    RTDPassportElementTypePersonalDetailsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypePersonalDetailsBuilder {
  inner: PassportElementTypePersonalDetails
}

impl RTDPassportElementTypePersonalDetailsBuilder {
  pub fn build(&self) -> PassportElementTypePersonalDetails { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypePersonalDetails> for PassportElementTypePersonalDetails {
  fn as_ref(&self) -> &PassportElementTypePersonalDetails { self }
}

impl AsRef<PassportElementTypePersonalDetails> for RTDPassportElementTypePersonalDetailsBuilder {
  fn as_ref(&self) -> &PassportElementTypePersonalDetails { &self.inner }
}







/// A Telegram Passport element containing the user's passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypePassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypePassport" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypePassport {}



impl PassportElementTypePassport {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypePassportBuilder {
    let mut inner = PassportElementTypePassport::default();
    inner.td_name = "passportElementTypePassport".to_string();
    RTDPassportElementTypePassportBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypePassportBuilder {
  inner: PassportElementTypePassport
}

impl RTDPassportElementTypePassportBuilder {
  pub fn build(&self) -> PassportElementTypePassport { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypePassport> for PassportElementTypePassport {
  fn as_ref(&self) -> &PassportElementTypePassport { self }
}

impl AsRef<PassportElementTypePassport> for RTDPassportElementTypePassportBuilder {
  fn as_ref(&self) -> &PassportElementTypePassport { &self.inner }
}







/// A Telegram Passport element containing the user's driver license
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeDriverLicense {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypeDriverLicense {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeDriverLicense" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypeDriverLicense {}



impl PassportElementTypeDriverLicense {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypeDriverLicenseBuilder {
    let mut inner = PassportElementTypeDriverLicense::default();
    inner.td_name = "passportElementTypeDriverLicense".to_string();
    RTDPassportElementTypeDriverLicenseBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypeDriverLicenseBuilder {
  inner: PassportElementTypeDriverLicense
}

impl RTDPassportElementTypeDriverLicenseBuilder {
  pub fn build(&self) -> PassportElementTypeDriverLicense { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypeDriverLicense> for PassportElementTypeDriverLicense {
  fn as_ref(&self) -> &PassportElementTypeDriverLicense { self }
}

impl AsRef<PassportElementTypeDriverLicense> for RTDPassportElementTypeDriverLicenseBuilder {
  fn as_ref(&self) -> &PassportElementTypeDriverLicense { &self.inner }
}







/// A Telegram Passport element containing the user's identity card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeIdentityCard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypeIdentityCard {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeIdentityCard" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypeIdentityCard {}



impl PassportElementTypeIdentityCard {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypeIdentityCardBuilder {
    let mut inner = PassportElementTypeIdentityCard::default();
    inner.td_name = "passportElementTypeIdentityCard".to_string();
    RTDPassportElementTypeIdentityCardBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypeIdentityCardBuilder {
  inner: PassportElementTypeIdentityCard
}

impl RTDPassportElementTypeIdentityCardBuilder {
  pub fn build(&self) -> PassportElementTypeIdentityCard { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypeIdentityCard> for PassportElementTypeIdentityCard {
  fn as_ref(&self) -> &PassportElementTypeIdentityCard { self }
}

impl AsRef<PassportElementTypeIdentityCard> for RTDPassportElementTypeIdentityCardBuilder {
  fn as_ref(&self) -> &PassportElementTypeIdentityCard { &self.inner }
}







/// A Telegram Passport element containing the user's internal passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeInternalPassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypeInternalPassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeInternalPassport" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypeInternalPassport {}



impl PassportElementTypeInternalPassport {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypeInternalPassportBuilder {
    let mut inner = PassportElementTypeInternalPassport::default();
    inner.td_name = "passportElementTypeInternalPassport".to_string();
    RTDPassportElementTypeInternalPassportBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypeInternalPassportBuilder {
  inner: PassportElementTypeInternalPassport
}

impl RTDPassportElementTypeInternalPassportBuilder {
  pub fn build(&self) -> PassportElementTypeInternalPassport { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypeInternalPassport> for PassportElementTypeInternalPassport {
  fn as_ref(&self) -> &PassportElementTypeInternalPassport { self }
}

impl AsRef<PassportElementTypeInternalPassport> for RTDPassportElementTypeInternalPassportBuilder {
  fn as_ref(&self) -> &PassportElementTypeInternalPassport { &self.inner }
}







/// A Telegram Passport element containing the user's address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypeAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeAddress" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypeAddress {}



impl PassportElementTypeAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypeAddressBuilder {
    let mut inner = PassportElementTypeAddress::default();
    inner.td_name = "passportElementTypeAddress".to_string();
    RTDPassportElementTypeAddressBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypeAddressBuilder {
  inner: PassportElementTypeAddress
}

impl RTDPassportElementTypeAddressBuilder {
  pub fn build(&self) -> PassportElementTypeAddress { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypeAddress> for PassportElementTypeAddress {
  fn as_ref(&self) -> &PassportElementTypeAddress { self }
}

impl AsRef<PassportElementTypeAddress> for RTDPassportElementTypeAddressBuilder {
  fn as_ref(&self) -> &PassportElementTypeAddress { &self.inner }
}







/// A Telegram Passport element containing the user's utility bill
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeUtilityBill {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypeUtilityBill {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeUtilityBill" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypeUtilityBill {}



impl PassportElementTypeUtilityBill {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypeUtilityBillBuilder {
    let mut inner = PassportElementTypeUtilityBill::default();
    inner.td_name = "passportElementTypeUtilityBill".to_string();
    RTDPassportElementTypeUtilityBillBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypeUtilityBillBuilder {
  inner: PassportElementTypeUtilityBill
}

impl RTDPassportElementTypeUtilityBillBuilder {
  pub fn build(&self) -> PassportElementTypeUtilityBill { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypeUtilityBill> for PassportElementTypeUtilityBill {
  fn as_ref(&self) -> &PassportElementTypeUtilityBill { self }
}

impl AsRef<PassportElementTypeUtilityBill> for RTDPassportElementTypeUtilityBillBuilder {
  fn as_ref(&self) -> &PassportElementTypeUtilityBill { &self.inner }
}







/// A Telegram Passport element containing the user's bank statement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeBankStatement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypeBankStatement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeBankStatement" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypeBankStatement {}



impl PassportElementTypeBankStatement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypeBankStatementBuilder {
    let mut inner = PassportElementTypeBankStatement::default();
    inner.td_name = "passportElementTypeBankStatement".to_string();
    RTDPassportElementTypeBankStatementBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypeBankStatementBuilder {
  inner: PassportElementTypeBankStatement
}

impl RTDPassportElementTypeBankStatementBuilder {
  pub fn build(&self) -> PassportElementTypeBankStatement { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypeBankStatement> for PassportElementTypeBankStatement {
  fn as_ref(&self) -> &PassportElementTypeBankStatement { self }
}

impl AsRef<PassportElementTypeBankStatement> for RTDPassportElementTypeBankStatementBuilder {
  fn as_ref(&self) -> &PassportElementTypeBankStatement { &self.inner }
}







/// A Telegram Passport element containing the user's rental agreement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeRentalAgreement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypeRentalAgreement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeRentalAgreement" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypeRentalAgreement {}



impl PassportElementTypeRentalAgreement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypeRentalAgreementBuilder {
    let mut inner = PassportElementTypeRentalAgreement::default();
    inner.td_name = "passportElementTypeRentalAgreement".to_string();
    RTDPassportElementTypeRentalAgreementBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypeRentalAgreementBuilder {
  inner: PassportElementTypeRentalAgreement
}

impl RTDPassportElementTypeRentalAgreementBuilder {
  pub fn build(&self) -> PassportElementTypeRentalAgreement { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypeRentalAgreement> for PassportElementTypeRentalAgreement {
  fn as_ref(&self) -> &PassportElementTypeRentalAgreement { self }
}

impl AsRef<PassportElementTypeRentalAgreement> for RTDPassportElementTypeRentalAgreementBuilder {
  fn as_ref(&self) -> &PassportElementTypeRentalAgreement { &self.inner }
}







/// A Telegram Passport element containing the registration page of the user's passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePassportRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypePassportRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypePassportRegistration" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypePassportRegistration {}



impl PassportElementTypePassportRegistration {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypePassportRegistrationBuilder {
    let mut inner = PassportElementTypePassportRegistration::default();
    inner.td_name = "passportElementTypePassportRegistration".to_string();
    RTDPassportElementTypePassportRegistrationBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypePassportRegistrationBuilder {
  inner: PassportElementTypePassportRegistration
}

impl RTDPassportElementTypePassportRegistrationBuilder {
  pub fn build(&self) -> PassportElementTypePassportRegistration { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypePassportRegistration> for PassportElementTypePassportRegistration {
  fn as_ref(&self) -> &PassportElementTypePassportRegistration { self }
}

impl AsRef<PassportElementTypePassportRegistration> for RTDPassportElementTypePassportRegistrationBuilder {
  fn as_ref(&self) -> &PassportElementTypePassportRegistration { &self.inner }
}







/// A Telegram Passport element containing the user's temporary registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeTemporaryRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypeTemporaryRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeTemporaryRegistration" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypeTemporaryRegistration {}



impl PassportElementTypeTemporaryRegistration {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypeTemporaryRegistrationBuilder {
    let mut inner = PassportElementTypeTemporaryRegistration::default();
    inner.td_name = "passportElementTypeTemporaryRegistration".to_string();
    RTDPassportElementTypeTemporaryRegistrationBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypeTemporaryRegistrationBuilder {
  inner: PassportElementTypeTemporaryRegistration
}

impl RTDPassportElementTypeTemporaryRegistrationBuilder {
  pub fn build(&self) -> PassportElementTypeTemporaryRegistration { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypeTemporaryRegistration> for PassportElementTypeTemporaryRegistration {
  fn as_ref(&self) -> &PassportElementTypeTemporaryRegistration { self }
}

impl AsRef<PassportElementTypeTemporaryRegistration> for RTDPassportElementTypeTemporaryRegistrationBuilder {
  fn as_ref(&self) -> &PassportElementTypeTemporaryRegistration { &self.inner }
}







/// A Telegram Passport element containing the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypePhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypePhoneNumber" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypePhoneNumber {}



impl PassportElementTypePhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypePhoneNumberBuilder {
    let mut inner = PassportElementTypePhoneNumber::default();
    inner.td_name = "passportElementTypePhoneNumber".to_string();
    RTDPassportElementTypePhoneNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypePhoneNumberBuilder {
  inner: PassportElementTypePhoneNumber
}

impl RTDPassportElementTypePhoneNumberBuilder {
  pub fn build(&self) -> PassportElementTypePhoneNumber { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypePhoneNumber> for PassportElementTypePhoneNumber {
  fn as_ref(&self) -> &PassportElementTypePhoneNumber { self }
}

impl AsRef<PassportElementTypePhoneNumber> for RTDPassportElementTypePhoneNumberBuilder {
  fn as_ref(&self) -> &PassportElementTypePhoneNumber { &self.inner }
}







/// A Telegram Passport element containing the user's email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PassportElementTypeEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeEmailAddress" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementType for PassportElementTypeEmailAddress {}



impl PassportElementTypeEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementTypeEmailAddressBuilder {
    let mut inner = PassportElementTypeEmailAddress::default();
    inner.td_name = "passportElementTypeEmailAddress".to_string();
    RTDPassportElementTypeEmailAddressBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementTypeEmailAddressBuilder {
  inner: PassportElementTypeEmailAddress
}

impl RTDPassportElementTypeEmailAddressBuilder {
  pub fn build(&self) -> PassportElementTypeEmailAddress { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PassportElementTypeEmailAddress> for PassportElementTypeEmailAddress {
  fn as_ref(&self) -> &PassportElementTypeEmailAddress { self }
}

impl AsRef<PassportElementTypeEmailAddress> for RTDPassportElementTypeEmailAddressBuilder {
  fn as_ref(&self) -> &PassportElementTypeEmailAddress { &self.inner }
}



