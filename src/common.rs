use ark_ff::PrimeField;
use ark_r1cs_std::{R1CSVar, alloc::AllocVar, fields::fp::FpVar};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

/// Return the slack between `from` and `to` if `from < to`, otherwise return zero.
pub fn get_slack<F: PrimeField>(
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
