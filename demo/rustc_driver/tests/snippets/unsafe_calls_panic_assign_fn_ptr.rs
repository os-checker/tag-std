#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(Safety)]
#![allow(dead_code)]

#[Safety::inner(Tag)]
unsafe fn call() {}

// Indirect call expressions are not supported yet.

pub fn assign_fn_ptr() {
    let f: unsafe fn() = call;
    unsafe {
        #[Safety::assign_fn_ptr(Tag)]
        f()
    };
}
