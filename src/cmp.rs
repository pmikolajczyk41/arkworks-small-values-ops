use ark_ff::PrimeField;
use ark_r1cs_std::{boolean::Boolean, eq::EqGadget, fields::fp::FpVar};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use crate::{common::enforce_zero, min};

/// Enforce that `a <= b`.
///
/// `a` and `b` must be in the range [0, 1 << `BITS`). `BITS` must be strictly less than the floor
/// of log2 of the field's modulus.
pub fn enforce_le<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    a: &FpVar<F>,
    b: &FpVar<F>,
) -> Result<(), SynthesisError> {
    assert!(BITS < (F::MODULUS_BIT_SIZE - 1) as usize);
    let less = min::<_, BITS>(cs, a, b)?;
    enforce_zero(&(a - less))
}

/// Enforce that `a >= b`.
///
/// `a` and `b` must be in the range [0, 1 << `BITS`). `BITS` must be strictly less than the floor
/// of log2 of the field's modulus.
pub fn enforce_ge<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    a: &FpVar<F>,
    b: &FpVar<F>,
) -> Result<(), SynthesisError> {
    enforce_le::<_, BITS>(cs, b, a)
}

/// Returns a Boolean indicating whether `a <= b`.
///
/// `a` and `b` must be in the range [0, 1 << `BITS`). `BITS` must be strictly less than the floor
/// of log2 of the field's modulus.
pub fn le<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    a: &FpVar<F>,
    b: &FpVar<F>,
) -> Result<Boolean<F>, SynthesisError> {
    let less = min::<_, BITS>(cs, a, b)?;
    a.is_eq(&less)
}

/// Returns a Boolean indicating whether `a >= b`.
///
/// `a` and `b` must be in the range [0, 1 << `BITS`). `BITS` must be strictly less than the floor
/// of log2 of the field's modulus.
pub fn ge<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    a: &FpVar<F>,
    b: &FpVar<F>,
) -> Result<Boolean<F>, SynthesisError> {
    le::<_, BITS>(cs, b, a)
}
