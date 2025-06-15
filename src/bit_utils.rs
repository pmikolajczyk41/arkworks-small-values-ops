use ark_ff::{BigInteger, PrimeField};
use ark_r1cs_std::{
    R1CSVar,
    alloc::AllocVar,
    boolean::Boolean,
    convert::ToConstraintFieldGadget,
    eq::EqGadget,
    fields::{FieldVar, fp::FpVar},
};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

/// Given a field element, get its `N` least significant bits as a vector of `Boolean`s in the
/// little-endian order.
///
/// `cs` is the constraint system reference used for creating the witness variables.
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

/// Given an array of `Boolean`s representing the bits of a field element in little-endian order,
/// reconstruct the field element.
pub fn from_bits<F: PrimeField>(bits: &[Boolean<F>]) -> Result<FpVar<F>, SynthesisError> {
    let mut value = FpVar::zero();
    for (i, bit) in bits.iter().enumerate() {
        value += FpVar::from(bit.clone()) * FpVar::constant(F::from(2).pow([i as u64]));
    }
    Ok(value)
}

/// Casts a field element to a `Boolean` variable, enforcing that the field element is either 0 or 1.
pub fn cast_to_boolean<F: PrimeField>(
    cs: ConstraintSystemRef<F>,
    value: &FpVar<F>,
) -> Result<Boolean<F>, SynthesisError> {
    let boolean = Boolean::new_witness(cs, || Ok(value.value()?.is_one()))?;
    boolean.to_constraint_field()?[0].enforce_equal(value)?;
    Ok(boolean)
}

/// Casts a field element to a `u64`, assuming the field element is in the range [0, 2^64).
pub fn cast_to_u64<F: PrimeField>(value: &FpVar<F>) -> Result<u64, SynthesisError> {
    let bigint = value.value()?.into_bigint();
    Ok(u64::from_le_bytes(
        bigint.to_bytes_le()[..8].try_into().unwrap(),
    ))
}
