pub struct Data {
    pub hash: PrimaryKey,
    pub func: Func,
}

pub struct PrimaryKey {
    pub hash1: u64,
    pub hash2: u64,
}

pub struct Func {
    /// Safety tool attributes
    pub tool_attrs: Vec<String>,
    /// Definition path (for debug purpose)
    pub def_path: String,
    /// Function source code without attributes (for debug purpose)
    pub function: String,
}
