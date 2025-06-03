use crate::{REGISTER_TOOL, Result};
use rustc_ast::MetaItemInner;
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::{
    AttrItem, Attribute, BodyId, FnSig, HirId, ImplItemKind, ItemKind, Node,
    def_id::{DefId, LocalDefId},
};
use rustc_middle::ty::TyCtxt;
use rustc_span::Ident;

mod db;
mod visit;

pub fn analyze_hir(tcx: TyCtxt) -> Result<()> {
    let mut v_hir_fn = Vec::with_capacity(64);
    let mut safety_attrs = FnToolAttrs::new(tcx);

    let def_items = tcx.hir_crate_items(()).definitions();
    for local_def_id in def_items {
        let node = tcx.hir_node_by_def_id(local_def_id);

        // fn item or associated fn item
        let hir_fn = match node {
            Node::Item(item) if matches!(item.kind, ItemKind::Fn { .. }) => {
                let (name, sig, _generics, body) = item.expect_fn();
                let sig = *sig;
                HirFn { local: local_def_id, hir_id: item.hir_id(), name, sig, body }
            }
            Node::ImplItem(item) if matches!(item.kind, ImplItemKind::Fn(..)) => {
                let (sig, body) = item.expect_fn();
                let hir_id = item.hir_id();
                HirFn { local: local_def_id, hir_id, name: item.ident, sig: *sig, body }
            }
            _ => continue,
        };

        v_hir_fn.push(hir_fn);
    }

    let data = {
        let mut db = db::Database::new("data.sqlite3")?;
        let iter = v_hir_fn.iter().map(|hir_fn| db::Data::new(hir_fn, tcx));
        db.save_data(iter)?;
        db.get_all_data()?
    };

    for hir_fn in &v_hir_fn {
        // look in the body
        let body = tcx.hir_body(hir_fn.body).value;
        let calls = visit::get_calls(tcx, body);
        let unsafe_calls = calls.get_unsafe_calls();
        if !unsafe_calls.is_empty() {
            dbg!(&unsafe_calls);
            for call in &unsafe_calls {
                call.get_all_attrs(hir_fn.hir_id, &mut safety_attrs);
            }
        }
    }

    Ok(())
}

fn is_tool_attr(attr: &&Attribute) -> bool {
    if let Attribute::Unparsed(tool_attr) = attr {
        if tool_attr.path.segments[0].as_str() == REGISTER_TOOL {
            return true;
        }
    }
    false
}

struct HirFn<'hir> {
    local: LocalDefId,
    hir_id: HirId,
    name: Ident,
    sig: FnSig<'hir>,
    body: BodyId,
}

struct FnToolAttrs<'tcx> {
    tcx: TyCtxt<'tcx>,
    map: FxIndexMap<DefId, Vec<SafetyAttr<'tcx>>>,
    /// State of safety tags shows if thet are discharged.
    tagged: FxIndexMap<Ident, bool>,
}

impl<'tcx> FnToolAttrs<'tcx> {
    fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self { tcx, map: FxIndexMap::default(), tagged: FxIndexMap::default() }
    }

    fn get_or_insert(&mut self, did: DefId) -> &[SafetyAttr<'tcx>] {
        self.map
            .entry(did)
            .or_insert_with(|| self.tcx.get_all_attrs(did).filter_map(SafetyAttr::new).collect())
    }

    fn get_or_insert_tags(&mut self, did: DefId) -> &mut FxIndexMap<Ident, bool> {
        self.tagged.clear();
        let entry = self.map.entry(did);
        let tags = entry
            .or_insert_with(|| self.tcx.get_all_attrs(did).filter_map(SafetyAttr::new).collect())
            .iter()
            .map(|attr| (attr.property, false));
        self.tagged.extend(tags);
        &mut self.tagged
    }
}

struct SafetyAttr<'tcx> {
    attr_item: &'tcx AttrItem,
    property: Ident,
}

impl<'tcx> SafetyAttr<'tcx> {
    fn new(attr: &Attribute) -> Option<SafetyAttr> {
        if let MetaItemInner::MetaItem(meta) = attr.meta_item_list()?.first()? {
            // #[Safety::path(Property)]
            let property = dbg!(meta).ident()?;
            dbg!(property);
            if let Attribute::Unparsed(tool_attr) = attr {
                if tool_attr.path.segments[0].as_str() == REGISTER_TOOL {
                    return Some(SafetyAttr { attr_item: tool_attr, property });
                }
            }
        }
        None
    }
}
