#[derive(derive_more::AsRef)]
struct Foo {
    #[as_ref]
    bar: i32,
    #[as_ref(ignore)]
    baz: f32,
}

fn main() {}
