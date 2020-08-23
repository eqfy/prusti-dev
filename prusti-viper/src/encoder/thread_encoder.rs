// © 2020, University of British Columbia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prusti_interface::environment::mir_analyses::initialization::{
    compute_definitely_initialized, DefinitelyInitializedAnalysisResult,
};
use prusti_interface::environment::place_set::PlaceSet;
use prusti_interface::environment::{BasicBlockIndex, PermissionForest, ProcedureLoops, Procedure};
use prusti_interface::utils;
use rustc_middle::{mir, ty};
use log::{trace, debug};

pub enum ThreadEncoderError {
    LoopInvariantInBranch(BasicBlockIndex),
}

pub struct ThreadEncoder<'p, 'tcx: 'p> {
    procedure: &'p Procedure<'p, 'tcx>,
    tcx: ty::TyCtxt<'tcx>,
    initialization: DefinitelyInitializedAnalysisResult<'tcx>,
}

impl<'p, 'tcx: 'p> ThreadEncoder<'p, 'tcx> {
    pub fn new(
        procedure: &'p Procedure<'p, 'tcx>,
        tcx: ty::TyCtxt<'tcx>,
    ) -> Self {
        ThreadEncoder {
            procedure,
            tcx,
            initialization: compute_definitely_initialized(
                procedure.get_mir(),
                tcx,
                tcx.hir().def_path(procedure.get_id().expect_local())
            ),
        }
    }

    pub fn mir(&self) -> &mir::Body<'tcx> {
        self.procedure.get_mir()
    }

}
