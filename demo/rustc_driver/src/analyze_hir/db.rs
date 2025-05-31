pub struct Data {
    pub hash: PrimaryKey,
    pub tool_attrs: ToolAttrs,
}

pub struct PrimaryKey {
    pub hash1: u64,
    pub hash2: u64,
}

pub struct ToolAttrs {
    pub inner: Vec<String>,
}
