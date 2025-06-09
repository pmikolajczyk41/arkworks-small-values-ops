use std::cmp::Ordering;

use ark_bn254::Fr;
use ark_ff::PrimeField;
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::fp::FpVar};
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef, SynthesisError};

fn main() -> Result<(), SynthesisError> {
    run_min(
        "Lib version",
        Fr::from(41),
        Fr::from(1729),
        arkworks_baby_gadgets::min::min::<Fr, 16>,
    )?;
    run_min("Standard version", Fr::from(41), Fr::from(1729), naive_min)?;

    Ok(())
}

fn run_min<Gadget>(header: &str, a: Fr, b: Fr, gadget: Gadget) -> Result<(), SynthesisError>
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

    report_cs(header, cs);
    Ok(())
}

fn report_cs(header: &str, cs: ConstraintSystemRef<Fr>) {
    assert!(cs.is_satisfied().unwrap());
    println!("=========={:=<20}==========", format!(" {header} "));
    println!("Constraints: {}", cs.num_constraints());
    println!(
        "Variables:   {}",
        cs.num_witness_variables() + cs.num_instance_variables()
    );
}

fn naive_min<F: PrimeField>(
    _cs: ConstraintSystemRef<F>,
    value: &FpVar<F>,
    cap: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    value.is_cmp(cap, Ordering::Less, false)?.select(value, cap)
}
