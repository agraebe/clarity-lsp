use std::collections::HashMap;

use crate::clarity::analysis::types::{ContractAnalysis, AnalysisPass};
use crate::clarity::analysis::AnalysisDatabase;
use crate::clarity::analysis::errors::{CheckResult, CheckError, CheckErrors};
use crate::clarity::representations::{SymbolicExpression, ClarityName};
use crate::clarity::representations::SymbolicExpressionType::{AtomValue, Atom, List, LiteralValue};
use crate::clarity::types::{Value, TraitIdentifier, TypeSignature, FunctionType};
use crate::clarity::functions::NativeFunctions;
use crate::clarity::functions::{DefineFunctions, DefineFunctionsParsed};

pub struct TraitChecker {
}

impl AnalysisPass for TraitChecker {

    fn run_pass(contract_analysis: &mut ContractAnalysis, analysis_db: &mut AnalysisDatabase) -> CheckResult<()> {
        let mut command = TraitChecker::new();
        command.run(contract_analysis, analysis_db)?;
        Ok(())
    }
}

impl TraitChecker {

    fn new() -> Self {
        Self {
        }
    }

    pub fn run(&mut self, contract_analysis: &mut ContractAnalysis, analysis_db: &mut AnalysisDatabase) -> CheckResult<()> {
    
        for trait_identifier in &contract_analysis.implemented_traits {

            let trait_name = trait_identifier.name.to_string();
            let contract_defining_trait = analysis_db.load_contract(&trait_identifier.contract_identifier)
                .ok_or(CheckErrors::TraitReferenceUnknown(trait_identifier.name.to_string()))?;
            
            let trait_definition = contract_defining_trait.get_defined_trait(&trait_name)
                .ok_or(CheckErrors::TraitReferenceUnknown(trait_identifier.name.to_string()))?;

            contract_analysis.check_trait_compliance(trait_identifier, trait_definition)?;
        }
        Ok(())
    }
}
