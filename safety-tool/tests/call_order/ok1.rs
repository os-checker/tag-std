#![feature(register_tool)]
#![register_tool(rapx)]

pub fn foo() {}

#[rapx::requires(PostToFunc(foo))]
pub unsafe fn bar() {}

pub fn test() {
    foo();
    unsafe {
        #[rapx::checked(PostToFunc(foo))]
        bar();
    }
}
