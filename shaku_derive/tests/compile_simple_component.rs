#![allow(dead_code)]

extern crate shaku;
#[macro_use]
extern crate shaku_derive;

#[derive(Component)]
#[interface(Foo)]
struct TestComponent {
    var1: String,
    var2: usize,
    var3: Box<String>,
    #[inject]
    var5: Box<dyn Bar>,
}

#[derive(Component)]
#[interface(Bar)]
struct BarImpl {
    val: usize,
}

trait Foo: Send {
    fn foo(&self);
}

trait Bar: Send {
    fn bar(&self);
}

impl Foo for TestComponent {
    fn foo(&self) {
        ()
    }
}

impl Bar for BarImpl {
    fn bar(&self) {
        ()
    }
}

#[test]
fn compile_ok() {}