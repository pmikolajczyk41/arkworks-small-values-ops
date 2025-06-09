use std::ops::Mul;

use ark_ff::PrimeField;
use ark_r1cs_std::{eq::EqGadget, fields::fp::FpVar};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use crate::{
    common::{enforce_zero, get_slack},
    enforce_in_binary_bound,
};

pub fn min<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    value: &FpVar<F>,
    cap: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    let over = get_slack(cs.clone(), cap, value)?;
    let undr = get_slack(cs.clone(), value, cap)?;

    // (1) Ensure that `over` and `undr` are within [0, 1 << BITS)
    enforce_in_binary_bound::<_, BITS>(&over)?;
    enforce_in_binary_bound::<_, BITS>(&undr)?;

    // (2) Ensure that `over` and `undr` are mutually exclusive
    enforce_zero(&over.clone().mul(&undr))?;

    // (3) Check the balance condition
    (value + undr).enforce_equal(&(cap + &over))?;

    Ok(value - over)
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

        let result = min::<_, BITS>(cs.clone(), &a_var, &b_var)?;
        assert_eq!(result.value()?, Fr::from(a.min(b)));

        Ok(())
    }

    #[test]
    fn test_min() -> Result<(), SynthesisError> {
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
