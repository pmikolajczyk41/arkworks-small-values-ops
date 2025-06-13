use ark_ff::PrimeField;
use ark_r1cs_std::fields::fp::FpVar;
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use crate::min;

/// Computes the saturating difference between two field elements `a` and `b`.
///
/// `a` and `b` must be in the range [0, 1 << `BITS`). `BITS` must be strictly less than the floor
/// of log2 of the field's modulus.
pub fn saturating_sub<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    a: &FpVar<F>,
    b: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    assert!(BITS < (F::MODULUS_BIT_SIZE - 1) as usize);
    let to_subtract = min::<_, BITS>(cs.clone(), a, b)?;
    Ok(a - to_subtract)
}
