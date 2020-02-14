use std::collections::HashMap;

use crate::clarity::analysis::types::{ContractAnalysis, AnalysisPass};
use crate::clarity::analysis::AnalysisDatabase;
use crate::clarity::analysis::errors::{CheckResult, CheckError, CheckErrors};
use crate::clarity::representations::{SymbolicExpression, ClarityName};
use crate::clarity::representations::SymbolicExpressionType::{AtomValue, Atom, List, LiteralValue};
use crate::clarity::types::{Value, TraitIdentifier, TypeSignature, FunctionType};
use crate::clarity::functions::NativeFunctions;
use crate::clarity::functions::{DefineFunctions, DefineFunctionsParsed};

pub struct PostTypeCheckingTraitChecker {
}

impl AnalysisPass for PostTypeCheckingTraitChecker {

    fn run_pass(contract_analysis: &mut ContractAnalysis, analysis_db: &mut AnalysisDatabase) -> CheckResult<()> {
        let mut command = PostTypeCheckingTraitChecker::new();
        command.run(contract_analysis, analysis_db)?;
        Ok(())
    }
}

impl PostTypeCheckingTraitChecker {

    fn new() -> Self {
        Self {
        }
    }

    pub fn run(&mut self, contract_analysis: &mut ContractAnalysis, analysis_db: &mut AnalysisDatabase) -> CheckResult<()> {
    
        for trait_identifier in &contract_analysis.implemented_traits {

            let trait_name = trait_identifier.name.to_string();
            let contract_defining_trait = analysis_db.load_contract(&trait_identifier.contract_identifier)
                .ok_or(CheckErrors::TraitReferenceUnknown(trait_identifier.name.to_string()))?;
            let trait_sig = contract_defining_trait.get_defined_trait(&trait_name)
                .ok_or(CheckErrors::TraitReferenceUnknown(trait_identifier.name.to_string()))?;

            for (func_name, expected_sig) in trait_sig.iter() {
                match contract_analysis.get_public_function_type(func_name) {
                    Some(FunctionType::Fixed(func)) => {
                        if func.args.len() != expected_sig.args.len() {
                            return Err(CheckErrors::BadTraitImplementation(trait_name.clone(), func_name.to_string()).into())
                        }
                        let args = expected_sig.args.iter().zip(func.args.iter());
                        for (expected_arg, arg) in args {
                            match (expected_arg, &arg.signature) {
                                (TypeSignature::TraitReferenceType(expected), TypeSignature::TraitReferenceType(actual)) => {
                                    let expected_trait_id = contract_defining_trait.get_referenced_trait(&expected.to_string())
                                        .ok_or(CheckErrors::BadTraitImplementation(trait_name.clone(), func_name.to_string()))?;
                                    let actual_trait_id = contract_analysis.get_referenced_trait(&actual.to_string())
                                        .ok_or(CheckErrors::BadTraitImplementation(trait_name.clone(), func_name.to_string()))?;
                                    if actual_trait_id != expected_trait_id {
                                        return Err(CheckErrors::BadTraitImplementation(trait_name.clone(), func_name.to_string()).into())
                                    }
                                }
                                _ => {
                                    if !expected_arg.admits_type(&arg.signature) {
                                        return Err(CheckErrors::BadTraitImplementation(trait_name.clone(), func_name.to_string()).into())
                                    }        
                                }
                            }
                        }

                        if !expected_sig.returns.admits_type(&func.returns) {
                            return Err(CheckErrors::BadTraitImplementation(trait_name, func_name.to_string()).into())
                        }
                    }
                    _ => {
                        return Err(CheckErrors::BadTraitImplementation(trait_name, func_name.to_string()).into())
                    }
                }
            }
        }
        Ok(())
    }
}