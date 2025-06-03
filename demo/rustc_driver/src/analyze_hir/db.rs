mod storage;

use super::{HirFn, is_tool_attr};
use jiff::Timestamp;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimaryKey {
    pub hash1: u64,
    pub hash2: u64,
}

impl PrimaryKey {
    fn new(def_id: DefId, tcx: TyCtxt) -> Self {
        let (hash1, hash2) = tcx.def_path_hash(def_id).0.split();
        PrimaryKey { hash1: hash1.as_u64(), hash2: hash2.as_u64() }
    }
}

pub struct Func {
    /// Safety tool attributes
    pub tool_attrs: Vec<String>,
    /// Definition path (for debug purpose)
    pub def_path: String,
    /// Function source code without attributes (for debug purpose)
    pub function: String,
}

pub struct Data {
    pub hash: PrimaryKey,
    pub func: Func,
}

impl Data {
    pub fn new(hir_fn: &HirFn, tcx: TyCtxt) -> Self {
        let def_id = hir_fn.local.to_def_id();
        let hash = PrimaryKey::new(def_id, tcx);

        let hid = hir_fn.hir_id;
        let func = Func {
            tool_attrs: tcx
                .hir_attrs(hid)
                .iter()
                .filter(is_tool_attr)
                .map(|attr| rustc_hir_pretty::attribute_to_string(&tcx, attr))
                .collect(),
            def_path: tcx.def_path_debug_str(def_id),
            function: rustc_hir_pretty::id_to_string(&tcx, hid),
        };

        Data { hash, func }
    }
}
