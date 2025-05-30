use rustc_hir::{Attribute, ItemKind};
use rustc_middle::ty::TyCtxt;

use crate::REGISTER_TOOL;

pub fn analyze_hir(tcx: TyCtxt) {
    let def_items = tcx.hir_crate_items(()).definitions();
    for local_def_id in def_items {
        let item = tcx.hir_expect_item(local_def_id);
        if let ItemKind::Fn { ident, .. } = item.kind {
            let attrs = tcx.hir_attrs(item.hir_id());

            let tool_attrs = attrs.iter().filter(|attr| {
                if let Attribute::Unparsed(tool_attr) = attr {
                    if tool_attr.path.segments[0].as_str() == REGISTER_TOOL {
                        return true;
                    }
                }
                false
            });
            for attr in tool_attrs {
                println!(
                    "{fn_name} ({span:?})\n => {attr:?}\n",
                    fn_name = ident,
                    span = item.vis_span,
                    attr = rustc_hir_pretty::attribute_to_string(&tcx, attr)
                );
            }
        }
    }
}
