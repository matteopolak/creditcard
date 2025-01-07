use std::{hint::black_box, str::FromStr};

use card_validate::Validate;
use creditcard::CreditCard;
use criterion::{Criterion, criterion_group, criterion_main};

fn too_short(c: &mut Criterion) {
	let mut group = c.benchmark_group("too short");

	group.bench_function("creditcard", |b| {
		b.iter(|| CreditCard::from_str(black_box("123456789012345")))
	});

	group.bench_function("card_validate", |b| {
		b.iter(|| Validate::from(black_box("123456789012345")))
	});

	group.finish();
}

fn too_long(c: &mut Criterion) {
	let mut group = c.benchmark_group("too long");

	group.bench_function("creditcard", |b| {
		b.iter(|| CreditCard::from_str(black_box("12345678901234567")))
	});

	group.bench_function("card_validate", |b| {
		b.iter(|| Validate::from(black_box("12345678901234567")))
	});

	group.finish();
}

fn invalid(c: &mut Criterion) {
	let mut group = c.benchmark_group("invalid");

	group.bench_function("creditcard", |b| {
		b.iter(|| CreditCard::from_str(black_box("1234567890123456")))
	});

	group.bench_function("card_validate", |b| {
		b.iter(|| Validate::from(black_box("1234567890123456")))
	});

	group.finish();
}

fn valid(c: &mut Criterion) {
	let mut group = c.benchmark_group("valid");

	group.bench_function("creditcard", |b| {
		b.iter(|| CreditCard::from_str(black_box("4111111111111111")))
	});

	group.bench_function("card_validate", |b| {
		b.iter(|| Validate::from(black_box("4111111111111111")))
	});

	group.finish();
}

criterion_group!(benches, too_short, too_long, invalid, valid);
criterion_main!(benches);
