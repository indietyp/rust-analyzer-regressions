#![feature(type_alias_impl_trait)]

type Foo = impl std::fmt::Debug;

async fn foo() -> Foo {
    42
}

fn main() {}
