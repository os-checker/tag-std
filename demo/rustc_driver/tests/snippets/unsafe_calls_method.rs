#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code)]

pub fn tag_expr() {
    let s = Struct::new();
    unsafe {
        #[Safety::tag_expr(property = Memo(Tag), kind = "memo")]
        s.call()
    };
}

pub fn tag_block() {
    let s = Struct::new();
    #[Safety::tag_block(property = Memo(Tag), kind = "memo")]
    unsafe {
        s.call();
    }
}

struct Struct {}

impl Struct {
    fn new() -> Self {
        Self {}
    }

    #[Safety::inner(property = Memo(Tag), kind = "memo")]
    unsafe fn call(&self) {}
}

#[Safety::tag_unsafe_fn(property = Memo(Tag), kind = "memo")]
pub unsafe fn tag_unsafe_fn() {
    let s = Struct::new();
    s.call();
}
