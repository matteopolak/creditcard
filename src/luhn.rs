//! Ported from https://github.com/valeriansaliou/rs-card-validate/blob/master/src/luhn.rs

/// `card` must be a slice of bytes within the range 0..=9.
pub(crate) fn is_valid(card: &[u8]) -> bool {
	let checksum = card
		.iter()
		.copied()
		.rev()
		.zip(1usize..)
		.fold(0, |mut checksum, (c, i)| {
			let is_odd = i % 2 == 1;

			if is_odd {
				checksum += checksum_modifier_odd(c);
			} else {
				checksum += checksum_modifier_even(c);
			};

			checksum
		});

	checksum % 10 == 0
}

#[inline(always)]
fn checksum_modifier_odd(c: u8) -> u32 {
	numeric_char_to_u32(c)
}

#[inline(always)]
fn checksum_modifier_even(c: u8) -> u32 {
	let n = numeric_char_to_u32(c);
	let d = n * 2;
	if d <= 9 { d } else { d - 9 }
}

#[inline(always)]
fn numeric_char_to_u32(c: u8) -> u32 {
	(c as u32) - ('0' as u32)
}
