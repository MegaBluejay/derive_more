#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code, clippy::unnecessary_mut_passed)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, collections::VecDeque, string::String, vec, vec::Vec};

#[cfg(feature = "std")]
use std::collections::VecDeque;

use core::ptr;

use derive_more::AsMut;

#[derive(AsMut)]
struct Foo(i32, f64, bool);

mod single_field {
    use super::*;

    mod tuple {
        use super::*;

        #[derive(AsMut)]
        struct Nothing(String);

        #[test]
        fn nothing() {
            let mut item = Nothing("test".to_owned());

            assert!(ptr::eq(item.as_mut(), &mut item.0));
        }

        #[derive(AsMut)]
        #[as_mut(forward)]
        struct Forward(String);

        #[test]
        fn forward() {
            let mut item = Forward("test".to_owned());

            let rf: &mut str = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));
        }

        #[derive(AsMut)]
        struct Field(#[as_mut] String);

        #[test]
        fn field() {
            let mut item = Field("test".to_owned());

            assert!(ptr::eq(item.as_mut(), &mut item.0));
        }

        #[derive(AsMut)]
        struct FieldForward(#[as_mut(forward)] String);

        #[test]
        fn field_forward() {
            let mut item = FieldForward("test".to_owned());

            let rf: &mut str = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));
        }

        #[derive(AsMut)]
        #[as_mut(i32, f64)]
        struct Types(Foo);

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for Types {
            fn as_mut(&mut self) -> &mut bool {
                self.0.as_mut()
            }
        }

        // Asserts that the macro expansion doesn't generate an `AsMut` impl for
        // the field type, by producing a trait implementations conflict error
        // during compilation, if it does.
        impl AsMut<Foo> for Types {
            fn as_mut(&mut self) -> &mut Foo {
                &mut self.0
            }
        }

        #[test]
        fn types() {
            let mut item = Types(Foo(1, 2.0, false));

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));

            let rf: &mut f64 = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));
        }

        #[derive(AsMut)]
        #[as_mut(i32, Foo)]
        struct TypesWithInner(Foo);

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for TypesWithInner {
            fn as_mut(&mut self) -> &mut bool {
                self.0.as_mut()
            }
        }

        #[test]
        fn types_with_inner() {
            let mut item = TypesWithInner(Foo(1, 2.0, false));

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));

            let rf: &mut Foo = item.as_mut();
            assert!(ptr::eq(rf, &mut item.0));
        }

        type RenamedFoo = Foo;

        #[derive(AsMut)]
        #[as_mut(i32, RenamedFoo)]
        struct TypesWithRenamedInner(Foo);

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.

        impl AsMut<bool> for TypesWithRenamedInner {
            fn as_mut(&mut self) -> &mut bool {
                self.0.as_mut()
            }
        }

        #[test]
        fn types_with_renamed_inner() {
            let mut item = TypesWithRenamedInner(Foo(1, 2.0, false));

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));

            let rf: &mut Foo = item.as_mut();
            assert!(ptr::eq(rf, &mut item.0));
        }

        #[derive(AsMut)]
        struct FieldTypes(#[as_mut(i32, f64)] Foo);

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for FieldTypes {
            fn as_mut(&mut self) -> &mut bool {
                self.0.as_mut()
            }
        }

        // Asserts that the macro expansion doesn't generate an `AsMut` impl for
        // the field type, by producing a trait implementations conflict error
        // during compilation, if it does.
        impl AsMut<Foo> for FieldTypes {
            fn as_mut(&mut self) -> &mut Foo {
                &mut self.0
            }
        }

        #[test]
        fn field_types() {
            let mut item = FieldTypes(Foo(1, 2.0, false));

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));

            let rf: &mut f64 = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));
        }

        #[derive(AsMut)]
        struct FieldTypesWithInner(#[as_mut(i32, Foo)] Foo);

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for FieldTypesWithInner {
            fn as_mut(&mut self) -> &mut bool {
                self.0.as_mut()
            }
        }

        #[test]
        fn field_types_with_inner() {
            let mut item = FieldTypesWithInner(Foo(1, 2.0, false));

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));

            let rf: &mut Foo = item.as_mut();
            assert!(ptr::eq(rf, &mut item.0));
        }

        #[derive(AsMut)]
        struct FieldTypesWithRenamedInner(#[as_mut(i32, RenamedFoo)] Foo);

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for FieldTypesWithRenamedInner {
            fn as_mut(&mut self) -> &mut bool {
                self.0.as_mut()
            }
        }

        #[test]
        fn field_types_with_renamed_inner() {
            let mut item = FieldTypesWithRenamedInner(Foo(1, 2.0, false));

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));

            let rf: &mut Foo = item.as_mut();
            assert!(ptr::eq(rf, &mut item.0));
        }

        mod generic {
            use super::*;

            #[derive(AsMut)]
            struct Nothing<T>(T);

            #[test]
            fn nothing() {
                let mut item = Nothing("test".to_owned());

                assert!(ptr::eq(item.as_mut(), &mut item.0));
            }

            #[derive(AsMut)]
            #[as_mut(forward)]
            struct Forward<T>(T);

            #[test]
            fn forward() {
                let mut item = Forward("test".to_owned());

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.0.as_mut()));
            }

            #[derive(AsMut)]
            struct Field<T>(#[as_mut] T);

            #[test]
            fn field() {
                let mut item = Field("test".to_owned());

                assert!(ptr::eq(item.as_mut(), &mut item.0));
            }

            #[derive(AsMut)]
            struct FieldForward<T>(#[as_mut(forward)] T);

            #[test]
            fn field_forward() {
                let mut item = FieldForward("test".to_owned());

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.0.as_mut()));
            }

            #[derive(AsMut)]
            #[as_mut(i32, f64)]
            struct Types<T>(T);

            // Asserts that the macro expansion doesn't generate a blanket `AsMut`
            // impl forwarding to the field type, by producing a trait implementations
            // conflict error during compilation, if it does.
            impl<T: AsMut<bool>> AsMut<bool> for Types<T> {
                fn as_mut(&mut self) -> &mut bool {
                    self.0.as_mut()
                }
            }

            #[test]
            fn types() {
                let mut item = Types(Foo(1, 2.0, false));

                let rf: &mut i32 = item.as_mut();
                assert!(ptr::eq(rf, item.0.as_mut()));

                let rf: &mut f64 = item.as_mut();
                assert!(ptr::eq(rf, item.0.as_mut()));
            }

            #[derive(AsMut)]
            #[as_mut(Vec<T>)]
            struct TypesInner<T>(Vec<T>);

            #[test]
            fn types_inner() {
                let mut item = TypesInner(vec![1i32]);

                assert!(ptr::eq(item.as_mut(), &mut item.0));
            }

            #[derive(AsMut)]
            struct FieldTypes<T>(#[as_mut(i32, f64)] T);

            // Asserts that the macro expansion doesn't generate a blanket `AsMut`
            // impl forwarding to the field type, by producing a trait implementations
            // conflict error during compilation, if it does.
            impl<T: AsMut<bool>> AsMut<bool> for FieldTypes<T> {
                fn as_mut(&mut self) -> &mut bool {
                    self.0.as_mut()
                }
            }

            #[test]
            fn field_types() {
                let mut item = FieldTypes(Foo(1, 2.0, false));

                let rf: &mut i32 = item.as_mut();
                assert!(ptr::eq(rf, item.0.as_mut()));

                let rf: &mut f64 = item.as_mut();
                assert!(ptr::eq(rf, item.0.as_mut()));
            }

            #[derive(AsMut)]
            struct FieldTypesInner<T>(#[as_mut(Vec<T>)] Vec<T>);

            #[test]
            fn field_types_inner() {
                let mut item = FieldTypesInner(vec![1i32]);

                assert!(ptr::eq(item.as_mut(), &mut item.0));
            }
        }
    }

    mod named {
        use super::*;

        #[derive(AsMut)]
        struct Nothing {
            first: String,
        }

        #[test]
        fn nothing() {
            let mut item = Nothing {
                first: "test".to_owned(),
            };

            assert!(ptr::eq(item.as_mut(), &mut item.first));
        }

        #[derive(AsMut)]
        #[as_mut(forward)]
        struct Forward {
            first: String,
        }

        #[test]
        fn forward() {
            let mut item = Forward {
                first: "test".to_owned(),
            };

            let rf: &mut str = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));
        }

        #[derive(AsMut)]
        struct Field {
            #[as_mut]
            first: String,
        }

        #[test]
        fn field() {
            let mut item = Field {
                first: "test".to_owned(),
            };

            assert!(ptr::eq(item.as_mut(), &mut item.first));
        }

        #[derive(AsMut)]
        struct FieldForward {
            #[as_mut(forward)]
            first: String,
        }

        #[test]
        fn field_forward() {
            let mut item = FieldForward {
                first: "test".to_owned(),
            };

            let rf: &mut str = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));
        }

        #[derive(AsMut)]
        #[as_mut(i32, f64)]
        struct Types {
            first: Foo,
        }

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for Types {
            fn as_mut(&mut self) -> &mut bool {
                self.first.as_mut()
            }
        }

        // Asserts that the macro expansion doesn't generate an `AsMut` impl for
        // the field type, by producing a trait implementations conflict error
        // during compilation, if it does.
        impl AsMut<Foo> for Types {
            fn as_mut(&mut self) -> &mut Foo {
                &mut self.first
            }
        }

        #[test]
        fn types() {
            let mut item = Types {
                first: Foo(1, 2.0, false),
            };

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));

            let rf: &mut f64 = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));
        }

        #[derive(AsMut)]
        #[as_mut(i32, Foo)]
        struct TypesWithInner {
            first: Foo,
        }

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for TypesWithInner {
            fn as_mut(&mut self) -> &mut bool {
                self.first.as_mut()
            }
        }

        #[test]
        fn types_with_inner() {
            let mut item = TypesWithInner {
                first: Foo(1, 2.0, false),
            };

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));

            let rf: &mut Foo = item.as_mut();
            assert!(ptr::eq(rf, &mut item.first));
        }

        type RenamedFoo = Foo;

        #[derive(AsMut)]
        #[as_mut(i32, RenamedFoo)]
        struct TypesWithRenamedInner {
            first: Foo,
        }

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for TypesWithRenamedInner {
            fn as_mut(&mut self) -> &mut bool {
                self.first.as_mut()
            }
        }

        #[test]
        fn types_with_renamed_inner() {
            let mut item = TypesWithRenamedInner {
                first: Foo(1, 2.0, false),
            };

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));

            let rf: &mut Foo = item.as_mut();
            assert!(ptr::eq(rf, &mut item.first));
        }

        #[derive(AsMut)]
        struct FieldTypes {
            #[as_mut(i32, f64)]
            first: Foo,
        }

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for FieldTypes {
            fn as_mut(&mut self) -> &mut bool {
                self.first.as_mut()
            }
        }

        // Asserts that the macro expansion doesn't generate an `AsMut` impl for
        // the field type, by producing a trait implementations conflict error
        // during compilation, if it does.
        impl AsMut<Foo> for FieldTypes {
            fn as_mut(&mut self) -> &mut Foo {
                &mut self.first
            }
        }

        #[test]
        fn field_types() {
            let mut item = FieldTypes {
                first: Foo(1, 2.0, false),
            };

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));

            let rf: &mut f64 = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));
        }

        #[derive(AsMut)]
        struct FieldTypesWithInner {
            #[as_mut(i32, Foo)]
            first: Foo,
        }

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for FieldTypesWithInner {
            fn as_mut(&mut self) -> &mut bool {
                self.first.as_mut()
            }
        }

        #[test]
        fn field_types_with_inner() {
            let mut item = FieldTypesWithInner {
                first: Foo(1, 2.0, false),
            };

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));

            let rf: &mut Foo = item.as_mut();
            assert!(ptr::eq(rf, &mut item.first));
        }

        #[derive(AsMut)]
        struct FieldTypesWithRenamedInner {
            #[as_mut(i32, RenamedFoo)]
            first: Foo,
        }

        // Asserts that the macro expansion doesn't generate a blanket `AsMut`
        // impl forwarding to the field type, by producing a trait implementations
        // conflict error during compilation, if it does.
        impl AsMut<bool> for FieldTypesWithRenamedInner {
            fn as_mut(&mut self) -> &mut bool {
                self.first.as_mut()
            }
        }

        #[test]
        fn field_types_with_renamed_inner() {
            let mut item = FieldTypesWithRenamedInner {
                first: Foo(1, 2.0, false),
            };

            let rf: &mut i32 = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));

            let rf: &mut Foo = item.as_mut();
            assert!(ptr::eq(rf, &mut item.first));
        }

        mod generic {
            use super::*;

            #[derive(AsMut)]
            struct Nothing<T> {
                first: T,
            }

            #[test]
            fn nothing() {
                let mut item = Nothing {
                    first: "test".to_owned(),
                };

                assert!(ptr::eq(item.as_mut(), &mut item.first));
            }

            #[derive(AsMut)]
            #[as_mut(forward)]
            struct Forward<T> {
                first: T,
            }

            #[test]
            fn struct_forward() {
                let mut item = Forward {
                    first: "test".to_owned(),
                };

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.first.as_mut()));
            }

            #[derive(AsMut)]
            struct Field<T> {
                #[as_mut]
                first: T,
            }

            #[test]
            fn field() {
                let mut item = Field {
                    first: "test".to_owned(),
                };

                assert!(ptr::eq(item.as_mut(), &mut item.first));
            }

            #[derive(AsMut)]
            struct FieldForward<T> {
                #[as_mut(forward)]
                first: T,
            }

            #[test]
            fn field_forward() {
                let mut item = FieldForward {
                    first: "test".to_owned(),
                };

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.first.as_mut()));
            }

            #[derive(AsMut)]
            #[as_mut(i32, f64)]
            struct Types<T> {
                first: T,
            }

            // Asserts that the macro expansion doesn't generate a blanket `AsMut`
            // impl forwarding to the field type, by producing a trait implementations
            // conflict error during compilation, if it does.
            impl<T: AsMut<bool>> AsMut<bool> for Types<T> {
                fn as_mut(&mut self) -> &mut bool {
                    self.first.as_mut()
                }
            }

            #[test]
            fn types() {
                let mut item = Types {
                    first: Foo(1, 2.0, false),
                };

                let rf: &mut i32 = item.as_mut();
                assert!(ptr::eq(rf, item.first.as_mut()));

                let rf: &mut f64 = item.as_mut();
                assert!(ptr::eq(rf, item.first.as_mut()));
            }

            #[derive(AsMut)]
            #[as_mut(Vec<T>)]
            struct TypesInner<T> {
                first: Vec<T>,
            }

            #[test]
            fn types_inner() {
                let mut item = TypesInner { first: vec![1i32] };

                assert!(ptr::eq(item.as_mut(), &mut item.first));
            }

            #[derive(AsMut)]
            struct FieldTypes<T> {
                #[as_mut(i32, f64)]
                first: T,
            }

            // Asserts that the macro expansion doesn't generate a blanket `AsMut`
            // impl forwarding to the field type, by producing a trait implementations
            // conflict error during compilation, if it does.
            impl<T: AsMut<bool>> AsMut<bool> for FieldTypes<T> {
                fn as_mut(&mut self) -> &mut bool {
                    self.first.as_mut()
                }
            }

            #[test]
            fn field_types() {
                let mut item = FieldTypes {
                    first: Foo(1, 2.0, false),
                };

                let rf: &mut i32 = item.as_mut();
                assert!(ptr::eq(rf, item.first.as_mut()));

                let rf: &mut f64 = item.as_mut();
                assert!(ptr::eq(rf, item.first.as_mut()));
            }

            #[derive(AsMut)]
            struct FieldTypesInner<T> {
                #[as_mut(Vec<T>)]
                first: Vec<T>,
            }

            #[test]
            fn field_types_inner() {
                let mut item = FieldTypesInner { first: vec![1i32] };

                assert!(ptr::eq(item.as_mut(), &mut item.first));
            }
        }
    }
}

mod multi_field {
    use super::*;

    mod tuple {
        use super::*;

        #[derive(AsMut)]
        struct Nothing(String, i32);

        #[test]
        fn nothing() {
            let mut item = Nothing("test".to_owned(), 0);

            assert!(ptr::eq(item.as_mut(), &mut item.0));
            assert!(ptr::eq(item.as_mut(), &mut item.1));
        }

        #[derive(AsMut)]
        struct Skip(String, i32, #[as_mut(skip)] f64);

        // Asserts that the macro expansion doesn't generate `AsMut` impl for the skipped field, by
        // producing trait implementations conflict error during compilation, if it does.
        impl AsMut<f64> for Skip {
            fn as_mut(&mut self) -> &mut f64 {
                &mut self.2
            }
        }

        #[test]
        fn skip() {
            let mut item = Skip("test".to_owned(), 0, 0.0);

            assert!(ptr::eq(item.as_mut(), &mut item.0));
            assert!(ptr::eq(item.as_mut(), &mut item.1));
        }

        #[derive(AsMut)]
        struct Field(#[as_mut] String, #[as_mut] i32, f64);

        // Asserts that the macro expansion doesn't generate `AsMut` impl for the third field, by
        // producing trait implementations conflict error during compilation, if it does.
        impl AsMut<f64> for Field {
            fn as_mut(&mut self) -> &mut f64 {
                &mut self.2
            }
        }

        #[test]
        fn field() {
            let mut item = Field("test".to_owned(), 0, 0.0);

            assert!(ptr::eq(item.as_mut(), &mut item.0));
            assert!(ptr::eq(item.as_mut(), &mut item.1));
        }

        #[derive(AsMut)]
        struct FieldForward(#[as_mut(forward)] String, i32);

        #[test]
        fn field_forward() {
            let mut item = FieldForward("test".to_owned(), 0);

            let rf: &mut str = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));
        }

        type RenamedString = String;

        #[derive(AsMut)]
        struct Types(
            #[as_mut(str, RenamedString)] String,
            #[as_mut([u8])] Vec<u8>,
        );

        // Asserts that the macro expansion doesn't generate `AsMut` impl for the field type, by
        // producing trait implementations conflict error during compilation, if it does.
        impl AsMut<Vec<u8>> for Types {
            fn as_mut(&mut self) -> &mut Vec<u8> {
                &mut self.1
            }
        }

        #[test]
        fn types() {
            let mut item = Types("test".to_owned(), vec![0]);

            let rf: &mut str = item.as_mut();
            assert!(ptr::eq(rf, item.0.as_mut()));

            let rf: &mut String = item.as_mut();
            assert!(ptr::eq(rf, &mut item.0));

            let rf: &mut [u8] = item.as_mut();
            assert!(ptr::eq(rf, item.1.as_mut()));
        }

        mod generic {
            use super::*;

            #[derive(AsMut)]
            struct Nothing<T, U>(Vec<T>, VecDeque<U>);

            #[test]
            fn nothing() {
                let mut item = Nothing(vec![1], VecDeque::from([2]));

                assert!(ptr::eq(item.as_mut(), &mut item.0));
                assert!(ptr::eq(item.as_mut(), &mut item.1));
            }

            #[derive(AsMut)]
            struct Skip<T, U, V>(Vec<T>, VecDeque<U>, #[as_mut(skip)] V);

            #[test]
            fn skip() {
                let mut item = Skip(vec![1], VecDeque::from([2]), 0);

                assert!(ptr::eq(item.as_mut(), &mut item.0));
                assert!(ptr::eq(item.as_mut(), &mut item.1));
            }

            #[derive(AsMut)]
            struct Field<T, U, V>(#[as_mut] Vec<T>, #[as_mut] VecDeque<U>, V);

            #[test]
            fn field() {
                let mut item = Field(vec![1], VecDeque::from([2]), 0);

                assert!(ptr::eq(item.as_mut(), &mut item.0));
                assert!(ptr::eq(item.as_mut(), &mut item.1));
            }

            #[derive(AsMut)]
            struct FieldForward<T, U>(#[as_mut(forward)] T, U);

            #[test]
            fn field_forward() {
                let mut item = FieldForward("test".to_owned(), 0);

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.0.as_mut()));
            }

            #[derive(AsMut)]
            struct Types<T, U>(#[as_mut(str)] T, #[as_mut([u8])] U);

            #[test]
            fn types() {
                let mut item = Types("test".to_owned(), vec![0]);

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.0.as_mut()));

                let rf: &mut [u8] = item.as_mut();
                assert!(ptr::eq(rf, item.1.as_mut()));
            }

            #[derive(AsMut)]
            struct TypesWithInner<T, U>(
                #[as_mut(Vec<T>, [T])] Vec<T>,
                #[as_mut(str)] U,
            );

            #[test]
            fn types_with_inner() {
                let mut item = TypesWithInner(vec![1i32], "a".to_owned());

                let rf: &mut Vec<i32> = item.as_mut();
                assert!(ptr::eq(rf, &mut item.0));

                let rf: &mut [i32] = item.as_mut();
                assert!(ptr::eq(rf, item.0.as_mut()));

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.1.as_mut()));
            }

            #[derive(AsMut)]
            struct FieldNonGeneric<T>(#[as_mut([T])] Vec<i32>, T);

            #[test]
            fn field_non_generic() {
                let mut item = FieldNonGeneric(vec![], 2i32);

                assert!(ptr::eq(item.as_mut(), item.0.as_mut()));
            }
        }
    }

    mod named {
        use super::*;

        #[derive(AsMut)]
        struct Nothing {
            first: String,
            second: i32,
        }

        #[test]
        fn nothing() {
            let mut item = Nothing {
                first: "test".to_owned(),
                second: 0,
            };

            assert!(ptr::eq(item.as_mut(), &mut item.first));
            assert!(ptr::eq(item.as_mut(), &mut item.second));
        }

        #[derive(AsMut)]
        struct Skip {
            first: String,
            second: i32,
            #[as_mut(skip)]
            third: f64,
        }

        // Asserts that the macro expansion doesn't generate `AsMut` impl for the skipped field, by
        // producing trait implementations conflict error during compilation, if it does.
        impl AsMut<f64> for Skip {
            fn as_mut(&mut self) -> &mut f64 {
                &mut self.third
            }
        }

        #[test]
        fn skip() {
            let mut item = Skip {
                first: "test".to_owned(),
                second: 0,
                third: 0.0,
            };

            assert!(ptr::eq(item.as_mut(), &mut item.first));
            assert!(ptr::eq(item.as_mut(), &mut item.second));
        }

        #[derive(AsMut)]
        struct Field {
            #[as_mut]
            first: String,
            #[as_mut]
            second: i32,
            third: f64,
        }

        // Asserts that the macro expansion doesn't generate `AsMut` impl for the `third` field, by
        // producing trait implementations conflict error during compilation, if it does.
        impl AsMut<f64> for Field {
            fn as_mut(&mut self) -> &mut f64 {
                &mut self.third
            }
        }

        #[test]
        fn field() {
            let mut item = Field {
                first: "test".to_owned(),
                second: 0,
                third: 0.0,
            };

            assert!(ptr::eq(item.as_mut(), &mut item.first));
            assert!(ptr::eq(item.as_mut(), &mut item.second));
        }

        #[derive(AsMut)]
        struct FieldForward {
            #[as_mut(forward)]
            first: String,
            second: i32,
        }

        #[test]
        fn field_forward() {
            let mut item = FieldForward {
                first: "test".to_owned(),
                second: 0,
            };

            let rf: &mut str = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));
        }

        type RenamedString = String;

        #[derive(AsMut)]
        struct Types {
            #[as_mut(str, RenamedString)]
            first: String,
            #[as_mut([u8])]
            second: Vec<u8>,
        }

        // Asserts that the macro expansion doesn't generate `AsMut` impl for unmentioned type, by
        // producing trait implementations conflict error during compilation, if it does.
        impl AsMut<Vec<u8>> for Types {
            fn as_mut(&mut self) -> &mut Vec<u8> {
                &mut self.second
            }
        }

        #[test]
        fn types() {
            let mut item = Types {
                first: "test".to_owned(),
                second: vec![0],
            };

            let rf: &mut str = item.as_mut();
            assert!(ptr::eq(rf, item.first.as_mut()));

            let rf: &mut String = item.as_mut();
            assert!(ptr::eq(rf, &mut item.first));

            let rf: &mut [u8] = item.as_mut();
            assert!(ptr::eq(rf, item.second.as_mut()));
        }

        mod generic {
            use super::*;

            #[derive(AsMut)]
            struct Nothing<T, U> {
                first: Vec<T>,
                second: VecDeque<U>,
            }

            #[test]
            fn nothing() {
                let mut item = Nothing {
                    first: vec![1],
                    second: VecDeque::from([2]),
                };

                assert!(ptr::eq(item.as_mut(), &mut item.first));
                assert!(ptr::eq(item.as_mut(), &mut item.second));
            }

            #[derive(AsMut)]
            struct Skip<T, U, V> {
                first: Vec<T>,
                second: VecDeque<U>,
                #[as_mut(skip)]
                third: V,
            }

            #[test]
            fn skip() {
                let mut item = Skip {
                    first: vec![1],
                    second: VecDeque::from([2]),
                    third: 0,
                };

                assert!(ptr::eq(item.as_mut(), &mut item.first));
                assert!(ptr::eq(item.as_mut(), &mut item.second));
            }

            #[derive(AsMut)]
            struct Field<T, U, V> {
                #[as_mut]
                first: Vec<T>,
                #[as_mut]
                second: VecDeque<U>,
                third: V,
            }

            #[test]
            fn field() {
                let mut item = Field {
                    first: vec![1],
                    second: VecDeque::from([2]),
                    third: 0,
                };

                assert!(ptr::eq(item.as_mut(), &mut item.first));
                assert!(ptr::eq(item.as_mut(), &mut item.second));
            }

            #[derive(AsMut)]
            struct FieldForward<T, U> {
                #[as_mut(forward)]
                first: T,
                second: U,
            }

            #[test]
            fn field_forward() {
                let mut item = FieldForward {
                    first: "test".to_owned(),
                    second: 0,
                };

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.first.as_mut()));
            }

            #[derive(AsMut)]
            struct Types<T, U> {
                #[as_mut(str)]
                first: T,
                #[as_mut([u8])]
                second: U,
            }

            #[test]
            fn types() {
                let mut item = Types {
                    first: "test".to_owned(),
                    second: vec![0],
                };

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.first.as_mut()));

                let rf: &mut [u8] = item.as_mut();
                assert!(ptr::eq(rf, item.second.as_mut()));
            }

            #[derive(AsMut)]
            struct TypesWithInner<T, U> {
                #[as_mut(Vec<T>, [T])]
                first: Vec<T>,
                #[as_mut(str)]
                second: U,
            }

            #[test]
            fn types_inner() {
                let mut item = TypesWithInner {
                    first: vec![1i32],
                    second: "a".to_owned(),
                };

                let rf: &mut Vec<i32> = item.as_mut();
                assert!(ptr::eq(rf, &mut item.first));

                let rf: &mut [i32] = item.as_mut();
                assert!(ptr::eq(rf, item.first.as_mut()));

                let rf: &mut str = item.as_mut();
                assert!(ptr::eq(rf, item.second.as_mut()));
            }

            #[derive(AsMut)]
            struct FieldNonGeneric<T> {
                #[as_mut([T])]
                first: Vec<i32>,
                second: T,
            }

            #[test]
            fn field_non_generic() {
                let mut item = FieldNonGeneric {
                    first: vec![],
                    second: 2i32,
                };

                assert!(ptr::eq(item.as_mut(), item.first.as_mut()));
            }
        }
    }
}
