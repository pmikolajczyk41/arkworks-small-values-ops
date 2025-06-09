use std::cmp::Ordering;

use ark_bn254::Fr;
use ark_ff::PrimeField;
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::fp::FpVar};
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef, SynthesisError};

struct Stats {
    constraints: usize,
    variables: usize,
}

fn main() -> Result<(), SynthesisError> {
    report_stats(&[
        (2, run_comparison::<2>()?),
        (4, run_comparison::<4>()?),
        (8, run_comparison::<8>()?),
        (16, run_comparison::<16>()?),
        (32, run_comparison::<32>()?),
        (64, run_comparison::<64>()?),
        (128, run_comparison::<128>()?),
        (250, run_comparison::<250>()?),
    ]);

    Ok(())
}

fn report_stats(comparisons: &[(usize, (Stats, Stats))]) {
    println!(
        "{:<6} {:>12} {:>12}    {:>12} {:>12}",
        "Bits", "LibConstr", "LibVars", "StdConstr", "StdVars"
    );
    println!("{}", "-".repeat(6 + 1 + 12 + 1 + 12 + 4 + 12 + 1 + 12));

    for &(bits, (ref lib, ref std)) in comparisons {
        println!(
            "{:<6} {:>12} {:>12}    {:>12} {:>12}",
            bits, lib.constraints, lib.variables, std.constraints, std.variables
        );
    }
}

fn run_comparison<const BITS: usize>() -> Result<(Stats, Stats), SynthesisError> {
    let (a, b) = (Fr::from(41), Fr::from(1729));
    let lib_stats = run_min(a, b, arkworks_small_values_ops::min::<Fr, BITS>)?;
    let naive_stats = run_min(a, b, naive_min)?;
    Ok((lib_stats, naive_stats))
}

fn run_min<Gadget>(a: Fr, b: Fr, gadget: Gadget) -> Result<Stats, SynthesisError>
where
    Gadget:
        Fn(ConstraintSystemRef<Fr>, &FpVar<Fr>, &FpVar<Fr>) -> Result<FpVar<Fr>, SynthesisError>,
{
    let cs = ConstraintSystem::new_ref();

    let value = FpVar::new_input(cs.clone(), || Ok(a))?;
    let cap = FpVar::new_input(cs.clone(), || Ok(b))?;
    let expected = FpVar::new_input(cs.clone(), || Ok(a.min(b)))?;

    let capped = gadget(cs.clone(), &value, &cap)?;
    expected.enforce_equal(&capped)?;

    Ok(Stats {
        constraints: cs.num_constraints(),
        variables: cs.num_witness_variables() + cs.num_instance_variables(),
    })
}

fn naive_min<F: PrimeField>(
    _cs: ConstraintSystemRef<F>,
    value: &FpVar<F>,
    cap: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    value.is_cmp(cap, Ordering::Less, false)?.select(value, cap)
}
