use std::any::{Any, type_name_of_val};

trait TestFun {}

impl dyn TestFun {
    fn foo(&self) {
        println!("foo");
    }
}

struct TestFunImpl;

impl TestFun for TestFunImpl {}

trait TypeName {
    fn type_name(&self) -> String;
}

impl<T> TypeName for T {
    fn type_name(&self) -> String {
        let name = type_name_of_val(self);
        println!("{}", name);
        String::from(name)
    }
}

fn main() {
    let boxed: Box<dyn TestFun> = Box::new(TestFunImpl);
    println!("{:?}", boxed.type_id());
    boxed.type_name();
}
