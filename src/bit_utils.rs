use ark_ff::{BigInteger, PrimeField};
use ark_r1cs_std::{
    R1CSVar,
    alloc::AllocVar,
    boolean::Boolean,
    fields::{FieldVar, fp::FpVar},
};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

pub fn to_bits<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    value: &FpVar<F>,
) -> Result<[Boolean<F>; BITS], SynthesisError> {
    let mut bits = [Boolean::FALSE; BITS];
    for (i, bit) in bits.iter_mut().enumerate() {
        *bit = Boolean::new_witness(cs.clone(), || Ok(value.value()?.into_bigint().get_bit(i)))?;
    }
    Ok(bits)
}

pub fn from_bits<F: PrimeField, const BITS: usize>(
    bits: &[Boolean<F>; BITS],
) -> Result<FpVar<F>, SynthesisError> {
    let mut value = FpVar::zero();
    for (i, bit) in bits.iter().enumerate() {
        value += FpVar::from(bit.clone()) * FpVar::constant(F::from(2).pow([i as u64]));
    }
    Ok(value)
}
