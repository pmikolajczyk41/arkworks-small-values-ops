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
