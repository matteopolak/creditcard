//! Ported from https://github.com/valeriansaliou/rs-card-validate/blob/master/src/lib.rs

#![no_std]

mod luhn;

use core::str::FromStr;

/// Common credit card issuers.
///
/// This list is not exhaustive and may not cover all issuers.
/// Since more may be added in the future, this has been marked
/// `#[non_exhaustive]`.
///
/// Taken from [Wikipedia](https://en.wikipedia.org/wiki/Payment_card_number).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Issuer {
	/// IIN ranges: 34, 37
	/// Length: 15
	AmericanExpress,
	/// IIN ranges: 31
	/// Length: 19
	ChinaTUnion,
	/// IIN ranges: 62
	/// Length: 16-19
	UnionPay,
	/// IIN ranges: 30, 36, 38, 39
	/// Length: 14-19
	DinersClub,
	/// IIN ranges: 6011, 644-649, 65, 622126-622925
	/// Length: 16-19
	Discover,
	/// IIN ranges: 60400100–60420099
	/// Length: 16-19
	UkrCard,
	/// IIN ranges: 60, 65, 81, 82, 508, 353, 356
	/// Length: 16
	RuPay,
	/// IIN ranges: 636
	/// Length: 16-19
	InterPayment,
	/// IIN ranges: 637-639
	/// Length: 16
	InstaPayment,
	/// IIN ranges: 3528–3589
	/// Length: 16-19
	Jcb,
	/// IIN ranges: 6759, 676770, 676774
	/// Length: 12-19
	MaestroUk,
	/// IIN ranges: 5018, 5020, 5038, 5893, 6304, 6761, 6762, 6763
	/// Length: 12-19
	Maestro,
	/// IIN ranges: 5019
	/// Length: 16
	Dankort,
	/// IIN ranges: 2200-2204
	/// Length: 16-19
	Mir,
	/// IIN ranges: 2205
	/// Length: 16
	Borica,
	/// IIN ranges: 2221–2720, 51-55
	/// Length: 16
	Mastercard,
	/// IIN ranges: 65, 9792
	/// Length: 16
	Troy,
	/// IIN ranges: 4
	/// Length: 13, 16, 19
	Visa,
	/// IIN ranges: 4026, 417500, 4508, 4844, 4913, 4917
	/// Length: 16
	VisaElectron,
	/// IIN ranges: 1
	/// Length: 15
	Uatp,
	/// IIN ranges: 506099–506198, 650002–650027, 507865–507964
	/// Length: 16, 18, 19
	Verve,
	/// IIN ranges: 357111
	/// Length: 16
	LankaPay,
	/// IIN ranges: 1946, 50, 56, 58, 60-63
	/// Length: 16, 18, 19
	Gpn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
	InvalidFormat,
	UnknownType,
	InvalidLength,
	InvalidLuhn,
}

impl Issuer {
	pub fn name(self) -> &'static str {
		use Issuer::*;

		match self {
			AmericanExpress => "American Express",
			ChinaTUnion => "China T-Union",
			UnionPay => "UnionPay",
			DinersClub => "Diners Club",
			Discover => "Discover",
			UkrCard => "UkrCard",
			RuPay => "RuPay",
			InterPayment => "InterPayment",
			InstaPayment => "InstaPayment",
			Jcb => "JCB",
			MaestroUk => "Maestro UK",
			Maestro => "Maestro",
			Dankort => "Dankort",
			Mir => "MIR",
			Borica => "Borica",
			Mastercard => "Mastercard",
			Troy => "Troy",
			Visa => "Visa",
			VisaElectron => "Visa Electron",
			Uatp => "UATP",
			Verve => "Verve",
			LankaPay => "LankaPay",
			Gpn => "GPN",
		}
	}

	fn is_length_valid(self, len: usize) -> bool {
		use Issuer::*;

		match self {
			AmericanExpress => len == 15,
			ChinaTUnion => len == 19,
			UnionPay => (16..=19).contains(&len),
			DinersClub => (14..=19).contains(&len),
			Discover => (16..=19).contains(&len),
			UkrCard => (16..=19).contains(&len),
			RuPay => len == 16,
			InterPayment => (16..=19).contains(&len),
			InstaPayment => len == 16,
			Jcb => (16..=19).contains(&len),
			MaestroUk => (12..=19).contains(&len),
			Maestro => (12..=19).contains(&len),
			Dankort => len == 16,
			Mir => (16..=19).contains(&len),
			Borica => len == 16,
			Mastercard => len == 16,
			Troy => len == 16,
			Visa => len == 13 || len == 16 || len == 19,
			VisaElectron => len == 16,
			Uatp => len == 15,
			Verve => len == 16 || len == 18 || len == 19,
			LankaPay => len == 16,
			Gpn => len == 16 || len == 18 || len == 19,
		}
	}
}

/// A credit card number.
///
/// # Example
///
/// ```
/// use creditcard::CreditCard;
///
/// let card = "4111111111111111".parse::<CreditCard>().unwrap();
///
/// assert_eq!(card.issuer(), creditcard::Issuer::Visa);
/// assert_eq!(card.pan(), 4111111111111111);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CreditCard {
	pan: u64,
	issuer: Issuer,
}

impl CreditCard {
	/// Parse a credit card number from a string.
	///
	/// Forwards to [`FromStr::from_str`].
	///
	/// # Example
	///
	/// ```
	/// use creditcard::CreditCard;
	///
	/// let card = CreditCard::parse_str("4111111111111111").unwrap();
	/// ```
	pub fn parse_str(card: &str) -> Result<CreditCard, Error> {
		card.parse()
	}

	/// The kind of credit card.
	///
	/// # Example
	///
	/// ```
	/// use creditcard::{CreditCard, Issuer};
	///
	/// let card = "4111111111111111".parse::<CreditCard>().unwrap();
	///
	/// assert_eq!(card.issuer(), Issuer::Visa);
	/// ```
	pub fn issuer(&self) -> Issuer {
		self.issuer
	}

	/// The credit card number.
	///
	/// # Example
	///
	/// ```
	/// use creditcard::CreditCard;
	///
	/// let card = "4111111111111111".parse::<CreditCard>().unwrap();
	///
	/// assert_eq!(card.pan(), 4111111111111111);
	/// ```
	pub fn pan(&self) -> u64 {
		self.pan
	}
}

impl FromStr for CreditCard {
	type Err = Error;

	fn from_str(card: &str) -> Result<Self, Self::Err> {
		let pan = card
			.parse::<u64>()
			.map_err(|_| Error::InvalidFormat)?;

		// all characters are ascii 0-9
		let bytes = card.as_bytes();

		if bytes.len() < 12 || bytes[0] == b'0' {
			return Err(Error::UnknownType);
		}

		// all IINs are at most 8 digits
		let iin = u32::from_str(&card[..8]).unwrap();

		// check in increase order of IIN length
		#[allow(clippy::inconsistent_digit_grouping)]
		let issuer = match iin {
			// 8
			60400100_..=60420099_ => Issuer::UkrCard,
			// 6
			506099_00..=506198_99 | 650002_00..=650027_99 | 507865_00..=507964_99 => {
				Issuer::Verve
			}
			622126_00..=622925_99 => Issuer::Discover,
			417500_00..=417500_99 => Issuer::VisaElectron,
			357111_00..=357111_99 => Issuer::LankaPay,
			676770_00..=676770_99 | 676774_00..=676774_99 => Issuer::MaestroUk,
			// 4
			6011_0000..=6011_9999 => Issuer::Discover,
			3528_0000..=3589_9999 => Issuer::Jcb,
			6759_0000..=6759_9999 => Issuer::MaestroUk,
			5018_0000..=5018_9999
			| 5020_0000..=5020_9999
			| 5038_0000..=5038_9999
			| 5893_0000..=5893_9999
			| 6304_0000..=6304_9999
			| 6761_0000..=6763_9999 => Issuer::Maestro,
			5019_0000..=5019_9999 => Issuer::Dankort,
			2200_0000..=2204_9999 => Issuer::Mir,
			2205_0000..=2205_9999 => Issuer::Borica,
			2221_0000..=2720_9999 => Issuer::Mastercard,
			9792_0000..=9792_9999 => Issuer::Troy,
			4026_0000..=4026_9999
			| 4508_0000..=4508_9999
			| 4844_0000..=4844_9999
			| 4913_0000..=4913_9999
			| 4917_0000..=4917_9999 => Issuer::VisaElectron,
			1946_0000..=1946_9999 => Issuer::Gpn,
			// 3
			644_00000..=649_99999 => Issuer::Discover,
			508_00000..=508_99999 => Issuer::RuPay,
			636_00000..=636_99999 => Issuer::InterPayment,
			637_00000..=639_99999 => Issuer::InstaPayment,
			// 2
			34_000000..=34_999999 | 37_000000..=37_999999 => Issuer::AmericanExpress,
			31_000000..=31_999999 => Issuer::ChinaTUnion,
			62_000000..=62_999999 => Issuer::UnionPay,
			30_000000..=30_999999
			| 36_000000..=36_999999
			| 38_000000..=38_999999
			| 39_000000..=39_999999 => Issuer::DinersClub,
			65_000000..=65_999999 => Issuer::Discover,
			60_000000..=60_999999 | 81_000000..=81_999999 | 82_000000..=82_999999 => {
				Issuer::RuPay
			}
			51_000000..=55_999999 => Issuer::Mastercard,
			50_000000..=50_999999
			| 56_000000..=56_999999
			| 58_000000..=58_999999
			| 60_000000..=63_999999 => Issuer::Gpn,
			// 1
			4_0000000..=4_9999999 => Issuer::Visa,
			1_0000000..=1_9999999 => Issuer::Uatp,
			_ => return Err(Error::UnknownType),
		};

		if !issuer.is_length_valid(bytes.len()) {
			return Err(Error::InvalidLength);
		}

		if !luhn::is_valid(bytes) {
			return Err(Error::InvalidLuhn);
		}

		Ok(CreditCard {
			pan,
			issuer,
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse_visa() {
		let card = "4111111111111111".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Visa);
		assert_eq!(card.pan(), 4111111111111111);

		let card = "4012888888881881".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Visa);
		assert_eq!(card.pan(), 4012888888881881);

		let card = "4222222222222".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Visa);
		assert_eq!(card.pan(), 4222222222222);
	}

	#[test]
	fn test_parse_mastercard() {
		let card = "5555555555554444".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Mastercard);
		assert_eq!(card.pan(), 5555555555554444);

		let card = "5105105105105100".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Mastercard);
		assert_eq!(card.pan(), 5105105105105100);
	}

	#[test]
	fn test_parse_amex() {
		let card = "378282246310005".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::AmericanExpress);
		assert_eq!(card.pan(), 378282246310005);

		let card = "371449635398431".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::AmericanExpress);
		assert_eq!(card.pan(), 371449635398431);
	}

	#[test]
	fn test_parse_discover() {
		let card = "6011111111111117".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Discover);
		assert_eq!(card.pan(), 6011111111111117);

		let card = "6011000990139424".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Discover);
		assert_eq!(card.pan(), 6011000990139424);
	}

	#[test]
	fn test_parse_diners_club() {
		let card = "30569309025904".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::DinersClub);
		assert_eq!(card.pan(), 30569309025904);

		let card = "38520000023237".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::DinersClub);
		assert_eq!(card.pan(), 38520000023237);
	}

	#[test]
	fn test_parse_jcb() {
		let card = "3530111333300000".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Jcb);
		assert_eq!(card.pan(), 3530111333300000);

		let card = "3566002020360505".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Jcb);
		assert_eq!(card.pan(), 3566002020360505);
	}

	#[test]
	fn test_parse_union_pay() {
		let card = "6200000000000005".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::UnionPay);
		assert_eq!(card.pan(), 6200000000000005);

		let card = "6200000000000000000".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::UnionPay);
		assert_eq!(card.pan(), 6200000000000000000);
	}

	#[test]
	fn test_parse_mir() {
		let card = "2200000000000004".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Mir);
		assert_eq!(card.pan(), 2200000000000004);

		let card = "2200999999999995".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Mir);
		assert_eq!(card.pan(), 2200999999999995);
	}

	#[test]
	fn test_parse_maestro_uk() {
		let card = "6759649826438453".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::MaestroUk);
		assert_eq!(card.pan(), 6759649826438453);

		let card = "6763990100000000015".parse::<CreditCard>().unwrap();

		assert_eq!(card.issuer(), Issuer::Maestro);
		assert_eq!(card.pan(), 6763990100000000015);
	}

	#[test]
	fn test_invalid_format() {
		let card = "4111111111111111a".parse::<CreditCard>();

		assert_eq!(card, Err(Error::InvalidFormat));
	}

	#[test]
	fn test_unknown_type() {
		let card = "0000000000000000".parse::<CreditCard>();

		assert_eq!(card, Err(Error::UnknownType));
	}

	#[test]
	fn test_invalid_length() {
		let card = "41111111111111111".parse::<CreditCard>();

		assert_eq!(card, Err(Error::InvalidLength));
	}

	#[test]
	fn test_invalid_luhn() {
		let card = "4111111111111112".parse::<CreditCard>();

		assert_eq!(card, Err(Error::InvalidLuhn));

		let card = "4111111111111113".parse::<CreditCard>();

		assert_eq!(card, Err(Error::InvalidLuhn));
	}
}
