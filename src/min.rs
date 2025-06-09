use std::ops::Mul;

use ark_ff::PrimeField;
use ark_r1cs_std::{
    alloc::AllocVar,
    boolean::Boolean,
    eq::EqGadget,
    fields::{fp::FpVar, FieldVar},
    R1CSVar,
};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use crate::bit_utils::{from_bits, to_bits};

pub fn min<F: PrimeField, const BITS: usize>(
    cs: ConstraintSystemRef<F>,
    value: &FpVar<F>,
    cap: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    let over = get_slack(cs.clone(), cap, value)?;
    let undr = get_slack(cs.clone(), value, cap)?;

    // (1) Ensure that `over` and `undr` are within [0, 1 << BITS)
    for x in [&over, &undr] {
        let bits = to_bits::<F, BITS>(cs.clone(), x)?;
        let reconstructed_from_bits = from_bits::<F, BITS>(&bits)?;
        reconstructed_from_bits.enforce_equal(x)?;
    }

    // (2) Ensure that `over` and `undr` are mutually exclusive
    (&over)
        .mul(&undr)
        .is_zero()?
        .enforce_equal(&Boolean::TRUE)?;

    // (3) Check the slack condition
    (value + undr).enforce_equal(&(cap + &over))?;

    Ok(value - over)
}

fn get_slack<F: PrimeField>(
    cs: ConstraintSystemRef<F>,
    from: &FpVar<F>,
    to: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    FpVar::new_witness(cs.clone(), || {
        let (from, to) = (from.value()?, to.value()?);
        if from < to {
            Ok(to - from)
        } else {
            Ok(F::zero())
        }
    })
}

#[cfg(test)]
mod tests {
    use ark_bn254::Fr;
    use ark_r1cs_std::fields::fp::FpVar;
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
