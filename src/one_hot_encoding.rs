use ark_ff::PrimeField;
use ark_r1cs_std::{
    alloc::AllocVar,
    eq::EqGadget,
    fields::{FieldVar, fp::FpVar},
};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use crate::{cast_to_u64, common::enforce_zero};

/// One-hot encodes a field element `x` into a vector of `BITS` FpVars.
///
/// We return FpVars, because usually the one-hot encoding is used for matrix-scalar multiplication,
/// where it is more convenient to work with FpVars directly, instead of Booleans.
///
/// `BITS` must be less than the field characteristic, otherwise the encoding is not valid.
pub fn one_hot_encode<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    x: &FpVar<F>,
) -> Result<[FpVar<F>; BITS], SynthesisError> {
    assert!(
        BITS < F::characteristic()[0] as usize,
        "Number of bits must be less than the field characteristic"
    );

    // 1) Convert the field element to an u64
    let x_u64 = cast_to_u64(x);

    // 2) Prepare the one-hot encoding witness
    let mut bits = vec![];
    for i in 0..BITS {
        let bit = FpVar::new_witness(cs.clone(), || Ok(F::from(i == x_u64? as usize)))?;
        bits.push(bit);
    }

    // 3) Constrain that this is a valid one-hot encoding
    // 3.1) Sum of bits must be 1 (exactly one bit is set)
    bits.iter()
        .fold(FpVar::zero(), |acc, bit| acc + bit)
        .enforce_equal(&FpVar::one())?;

    // 3.2) The xth bit must be set.
    for (i, bit) in bits.iter().enumerate() {
        enforce_zero(&(bit * (x - FpVar::Constant(F::from(i as u64)))))?;
    }

    Ok(bits.try_into().expect("Size mismatch in one-hot encoding"))
}
