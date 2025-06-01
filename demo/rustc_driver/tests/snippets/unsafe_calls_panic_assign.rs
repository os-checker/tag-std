#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code)]

#[Safety::inner(Tag)]
unsafe fn call() {}

// Indirect call expressions are not supported yet.

pub fn assign() {
    let f = call;
    #[Safety::assign(Tag)]
    unsafe {
        f()
    };
}
