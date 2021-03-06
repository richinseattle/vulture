//! Analyses and Optimizations over Vulture IL

pub mod analysis_location;
mod dead_code_elimination;
mod def_use;
mod fixed_point;
mod reaching_definitions;
mod value_set;

use error::*;
use il;
use self::analysis_location::AnalysisLocation;
pub use self::reaching_definitions::Reaches;
use std::collections::{BTreeMap, BTreeSet};

/// `Analysis` holds several types of analysis results.
///
/// `Analysis` is unable to modify its `ControlFlowGraph`, which gaurantees the
/// analyses returns are valid of the `ControlFlowGraph` it was given. In the
/// event an optimization should need to modify the `ControlFlowGraph`, the
/// `ControlFlowGraph` will be copied, and the new, modified graph returned.
pub struct Analysis<'a> {
    control_flow_graph: &'a il::ControlFlowGraph,
    reaching_definitions: BTreeMap<AnalysisLocation, Reaches>,
    def_use: BTreeMap<AnalysisLocation, BTreeSet<AnalysisLocation>>,
    use_def: BTreeMap<AnalysisLocation, BTreeSet<AnalysisLocation>>,
}


impl<'a> Analysis<'a> {
    pub fn initialize(control_flow_graph: &'a il::ControlFlowGraph)
    -> Result<Analysis<'a>> {
        let rd = reaching_definitions::compute(control_flow_graph)?;
        let du = def_use::def_use(&rd, control_flow_graph)?;
        let ud = def_use::use_def(&rd, control_flow_graph)?;
        Ok(Analysis {
            control_flow_graph: control_flow_graph,
            reaching_definitions: rd,
            def_use: du,
            use_def: ud
        })
    }

    /// Returns the ControlFlowGraph all analysis was performed over.
    pub fn control_flow_graph(&self) -> &il::ControlFlowGraph {
        &self.control_flow_graph
    }

    /// Reaching definitions for this `Analysis`.
    pub fn reaching_definitions(&self) -> &BTreeMap<AnalysisLocation, Reaches> {
        &self.reaching_definitions
    }

    /// Def Use chains for this `Analysis`.
    pub fn def_use(&self) -> &BTreeMap<AnalysisLocation, BTreeSet<AnalysisLocation>> {
        &self.def_use
    }

    /// Use Def chains for this `Analysis`.
    pub fn use_def(&self) -> &BTreeMap<AnalysisLocation, BTreeSet<AnalysisLocation>> {
        &self.use_def
    }

    /// Performs dead code elimination and returns the result in a new graph.
    pub fn dead_code_elimination(&self) -> Result<il::ControlFlowGraph> {
        dead_code_elimination::dead_code_elimination(self)
    }
}