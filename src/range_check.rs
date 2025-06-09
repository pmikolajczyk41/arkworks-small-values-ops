use ark_ff::PrimeField;
use ark_r1cs_std::{
    R1CSVar,
    eq::EqGadget,
    fields::{FieldVar, fp::FpVar},
};
use ark_relations::r1cs::SynthesisError;

use crate::{common::enforce_zero, from_bits, to_bits};

/// Naively(!) enforces that the given value is in the set defined by the provided elements. It is
/// done by constructing a vanishing polynomial.
///
/// WARNING: Use this only for small sets - the degree of the polynomial is linear in the size of
/// the set.
pub fn enforce_in_set<F: PrimeField>(
    value: &FpVar<F>,
    set: &[FpVar<F>],
) -> Result<(), SynthesisError> {
    let mut vanishing = FpVar::one();
    for element in set {
        vanishing *= value - element;
    }
    enforce_zero(&vanishing)
}

/// (Naively!) enforces that the given value is in the range [0, bound] (inclusive!). This is done
/// by delegating to `enforce_in_set` with the set of all integers in that range.
///
/// WARNING: Use this only for small bounds (this is why `bound` is limited to `u8`).
pub fn enforce_in_bound<F: PrimeField>(value: &FpVar<F>, bound: u8) -> Result<(), SynthesisError> {
    let set = (0..=bound)
        .map(|i| FpVar::constant(F::from(i)))
        .collect::<Vec<_>>();
    enforce_in_set(value, &set)
}

/// Enforces that the given value is in the range [0, 2^BITS - 1] (inclusive!). This is done by
/// introducing `BITS` new boolean variables and reconstructing the value from them.
pub fn enforce_in_binary_bound<F: PrimeField, const BITS: usize>(
    value: &FpVar<F>,
) -> Result<(), SynthesisError> {
    let bit_decomposition = to_bits::<_, BITS>(value.cs().clone(), value)?;
    let reconstruction = from_bits(&bit_decomposition)?;
    reconstruction.enforce_equal(value)
}
