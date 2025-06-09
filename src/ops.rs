use std::ops::Mul;

use ark_ff::PrimeField;
use ark_r1cs_std::{eq::EqGadget, fields::fp::FpVar};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use crate::{
    common::{enforce_zero, get_slack},
    enforce_in_binary_bound,
};

/// Computes the minimum of two field elements `a` and `b` using slack variables to ensure that the
/// result is correct without directly comparing the two values.
///
/// `a` and `b` must be in the range [0, 1 << `BITS`). `BITS` must be strictly less than the floor
/// of log2 of the field's modulus.
pub fn min<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    a: &FpVar<F>,
    b: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    assert!(BITS < (F::MODULUS_BIT_SIZE - 1) as usize);
    let (_undr, over) = get_under_and_over_checked::<F, BITS>(cs, a, b)?;
    Ok(a - over)
}

/// Computes the maximum of two field elements `a` and `b` using slack variables to ensure that the
/// result is correct without directly comparing the two values.
///
/// `a` and `b` must be in the range [0, 1 << `BITS`). `BITS` must be strictly less than the floor
/// of log2 of the field's modulus.
pub fn max<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    a: &FpVar<F>,
    b: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    assert!(BITS < (F::MODULUS_BIT_SIZE - 1) as usize);
    let (undr, _over) = get_under_and_over_checked::<F, BITS>(cs, a, b)?;
    Ok(a + undr)
}

fn get_under_and_over_checked<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    a: &FpVar<F>,
    b: &FpVar<F>,
) -> Result<(FpVar<F>, FpVar<F>), SynthesisError> {
    let over = get_slack(cs.clone(), b, a)?;
    let undr = get_slack(cs.clone(), a, b)?;

    // (1) Ensure that `over` and `undr` are within [0, 1 << BITS)
    enforce_in_binary_bound::<_, BITS>(&over)?;
    enforce_in_binary_bound::<_, BITS>(&undr)?;

    // (2) Ensure that `over` and `undr` are mutually exclusive
    enforce_zero(&over.clone().mul(&undr))?;

    // (3) Check the balance condition
    (a + &undr).enforce_equal(&(b + &over))?;

    Ok((undr, over))
}

#[cfg(test)]
mod tests {
    use ark_bn254::Fr;
    use ark_r1cs_std::{R1CSVar, alloc::AllocVar, fields::fp::FpVar};
    use ark_relations::r1cs::ConstraintSystem;

    use super::*;

    fn run<const BITS: usize>(a: u64, b: u64) -> Result<(), SynthesisError> {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let a_var = FpVar::new_witness(cs.clone(), || Ok(Fr::from(a)))?;
        let b_var = FpVar::new_witness(cs.clone(), || Ok(Fr::from(b)))?;

        let min_result = min::<_, BITS>(cs.clone(), &a_var, &b_var)?;
        assert_eq!(min_result.value()?, Fr::from(a.min(b)));

        let max_result = max::<_, BITS>(cs.clone(), &a_var, &b_var)?;
        assert_eq!(max_result.value()?, Fr::from(a.max(b)));

        Ok(())
    }

    #[test]
    fn test() -> Result<(), SynthesisError> {
        // Small values
        run::<3>(3, 5)?;
        run::<3>(5, 3)?;

        // Equal values
        run::<3>(5, 5)?;

        // Zero values
        run::<3>(0, 5)?;
        run::<3>(5, 0)?;
        run::<3>(0, 0)?;

        // Larger values
        run::<64>(123456789, 234567890)?;

        Ok(())
    }
}
