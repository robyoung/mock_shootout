use simulacrum::*;

/// ```
/// use simulacrum::*;
///
/// pub trait A {
///     fn foo(&self, key: i16) -> i32;
/// }
///
/// create_mock! {
///     impl A for AMock (self) {
///         expect_foo("foo"):
///         fn foo(&self, key: i16) -> i32;
///     }
/// }
///
/// let mut mock = AMock::new();
/// mock.expect_foo().called_once().with(-1).returning(|_| 42);
/// assert_eq!(42, mock.foo(-1));
/// ```
fn doctest() {}

struct Bean();
impl Bean {
    pub fn eat(&self) {}
}
struct BeanMock{
    e: Expectations
}
impl BeanMock {
    pub fn new() -> Self {
        Self {
            e: Expectations::new()
        }
    }
    pub fn eat(&self) {
        self.e.was_called::<(), ()>("eat", ())
    }
    pub fn expect_eat(&mut self) -> Method<(), ()> {
        self.e.expect::<(), ()>("eat")
    }
    pub fn then(&mut self) -> &mut Self {
        self.e.then();
        self
    }
}

#[cfg(test)]
mod t {

use simulacrum::*;
use simulacrum_user::{deref, gt, lt, passes};
use crate::TestSuite;
use test_double::*;
#[test_double] use super::Bean;

struct Simulacrum {}
impl TestSuite for Simulacrum {
    const NAME: &'static str = "simulacrum";

    fn associated_types() {
        // Traits with associated types can be mocked more easily than Generic
        // Traits.
        pub trait A {
            type Key;
            type Value;
            fn foo(&self, k: Self::Key) -> Self::Value;
        }

        create_mock_struct! {
            struct AMock: {
                expect_foo("foo") i16 => u32;
            }
        }

        impl A for AMock {
            type Key=i16;
            type Value=u32;

            fn foo(&self, k: Self::Key) -> Self::Value {
                was_called!(self, "foo", (k: i16) -> u32)
            }
        }

        let mut mock: AMock = AMock::new();
        mock.expect_foo().called_once().with(-1).returning(|_| 5);

        assert_eq!(5, mock.foo(-1));
    }

    fn checkpoint() {
        pub trait A {
            fn foo(&self);
            fn bar(&self);
            fn baz(&self);
            fn bang(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
                expect_bar("bar"):
                fn bar(&self);
                expect_baz("baz"):
                fn baz(&self);
                expect_bang("bang"):
                fn bang(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once();
        mock.expect_bar().called_once();
        mock.then().expect_baz().called_once();
        mock.expect_bang().called_once();

        mock.foo();
        mock.bar();
        fn mock_trait() {
        }

        mock.bang();
        mock.baz();
    }

    // To mock generic methods Simulacrum requires naming each concrete type
    // that will be used with the method.  But that's usually not possible for
    // closures.
    fn closures() { unimplemented!() }

    fn reference_parameters() {
        // Simulacrum can do this, but it needs unsafe code
        pub trait A {
            fn foo(&self, x: &u32);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: &u32);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any().with(passes(
                |x: &*const u32| unsafe { **x == 1 }
        ));

        mock.foo(&1);
    }

    fn consume_parameters() {
        // Simulacrum's returning and modifying methods take their parameters by
        // reference
        unimplemented!()
    }

    fn consume_self() {
        pub trait A {
            fn foo(self);
        }

        create_mock_struct! {
            struct AMock: {
                expect_foo("foo");
            }
        }

        impl A for AMock {
            fn foo(self) {
                was_called!(self, "foo")
            }
        }

        let mut mock = AMock::new();
        mock.expect_foo().called_once();

        mock.foo();
    }

    fn derive() {
        // Simulacrum does not yet support Deriving mocks.  That feature is
        // planned for the upcoming simulacrum_auto crate
        unimplemented!()
    }

    fn external_trait() {
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let _mock = AMock::new();
    }

    fn fallback() {
        // Simulacrum lacks this capability.  In some cases, it can be
        // implemented with returning and a lambda.  But that doesn't always
        // work, because returning supplies its lambda with function arguments
        // by reference instead of by value.
        unimplemented!()
    }

    fn foreign() { unimplemented!() }
    // Simulacrum can't mock a generic method with different parameter types
    // more than once in the same mock object, at least not using the normal
    // syntax.  But there's a workaround for manually-constructed mock objects.
    // https://github.com/pcsm/simulacrum/issues/55
    fn generic_method() {
        pub trait A {
            fn foo<T: 'static>(&self, t:T);
        }

        create_mock_struct! {
            struct AMock: {
                expect_foo_i16("foo") i16;
                expect_foo_u32("foo") u32;
            }
        }

        impl A for AMock {
            fn foo<T: 'static>(&self, t: T) {
                was_called!(self, "foo", (t: T))
            }
        }

        let mut mock: AMock = AMock::new();
        mock.expect_foo_i16().called_once().with(-1);
        mock.then().expect_foo_u32().called_once().with(1);

        mock.foo::<i16>(-1);
        mock.foo::<u32>(1);
    }

    fn generic_return() {
        pub trait A {
            fn bar<T: 'static>(&self) -> T;
        }

        create_mock_struct! {
            struct AMock: {
                expect_bar_i16("bar") () => i16;
                expect_bar_u32("bar") () => u32;
            }
        }

        impl A for AMock {
            fn bar<T: 'static>(&self) -> T {
                was_called!(self, "bar", () -> T)
            }
        }

        let mut mock: AMock = AMock::new();
        mock.expect_bar_i16().called_once().returning(|_| -5);
        mock.then().expect_bar_u32().called_once().returning(|_| 1_000_000);

        assert_eq!(-5, mock.bar::<i16>());
        assert_eq!(1_000_000, mock.bar::<u32>());
    }

    fn generic_struct() {
        let mut mock = Bean::new();
        mock.expect_eat().called_once();
        mock.eat();
    }

    fn generic_trait() {
        // Generic Traits can be mocked using Simulacrum's mid-level macros.
        // But the Mock struct will be concrete, not generic.
        pub trait A<T> {
            fn foo(&self, t: T) -> u32;
        }

        create_mock_struct! {
            struct AMock: {
                expect_foo("foo") i16 => u32;
            }
        }

        impl A<i16> for AMock {
            fn foo(&self, t: i16) -> u32 {
                was_called!(self, "foo", (t: i16) -> u32)
            }
        }

        let mut mock: AMock = AMock::new();
        mock.expect_foo().called_once().with(-1).returning(|_| 5);

        assert_eq!(5, mock.foo(-1));
    }

    // Can't derive mocks for structs
    fn impl_trait() {unimplemented!() }

    fn inherited_trait() {
        // Simulacrum can mock inherited traits using mid-level macros
        pub trait A {
            fn foo(&self) -> u32;
        }
        pub trait B: A {
            fn bar(&self) -> u32;
        }

        create_mock_struct! {
            struct BMock: {
                expect_foo("foo") () => u32;
                expect_bar("bar") () => u32;
            }
        }
        impl A for BMock {
            fn foo(&self) -> u32 {
                was_called!(self, "foo", () -> u32)
            }
        }
        impl B for BMock {
            fn bar(&self) -> u32 {
                was_called!(self, "bar", () -> u32)
            }
         }

        let mut mock = BMock::new();
        mock.expect_foo().called_any().returning(|_| 42);
        mock.expect_bar().called_any().returning(|_| 99);

        assert_eq!(42, mock.foo());
        assert_eq!(99, mock.bar());
    }

    fn many_args() {
        pub trait A {
            fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                h: i8, i: i8);
        }
        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                h: i8, i: i8);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any().with(params!(0, 1, 2, 3, 4, 5, 6, 7, 8));
        mock.foo(0, 1, 2, 3, 4, 5, 6, 7, 8);
        print!("9 ");
        // Simulacrum's params! macro works with a maximum of 9 arguments
    }

    fn match_combo() { unimplemented!() }
    fn match_constant() {
        pub trait A {
            fn foo(&self, x: u32);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: u32);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any().with(42);

        mock.foo(42);
    }

    fn match_method() {
        pub trait A {
            fn foo(&self, x: u32);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: u32);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any().with(passes(|x| *x == 42));

        mock.foo(42);
    }

    fn match_operator() {
        // Simulacrum doesn't have any ge, le, or ne operators
        pub trait A {
            fn foo_deref(&self, key: &i16);
            fn foo_eq(&self, key: i16);
            fn foo_gt(&self, key: i16);
            fn foo_lt(&self, key: i16);
        }

        create_mock! {
            impl A for AMock(self) {
                expect_foo_deref("foo_deref"):
                fn foo_deref(&self, key: &i16);
                expect_foo_eq("foo_eq"):
                fn foo_eq(&self, key: i16);
                expect_foo_gt("foo_gt"):
                fn foo_gt(&self, key: i16);
                expect_foo_lt("foo_lt"):
                fn foo_lt(&self, key: i16);
            }
        }

        let mut mock = AMock::new();
        mock.expect_foo_deref().called_any().with(deref(3));
        mock.expect_foo_eq().called_any().with(3);
        mock.expect_foo_gt().called_any().with(gt(2));
        mock.expect_foo_lt().called_any().with(lt(4));
        mock.foo_deref(&3);
        mock.foo_eq(3);
        mock.foo_gt(3);
        mock.foo_lt(3);
    }

    fn match_pattern() { unimplemented!() }
    fn match_range() { unimplemented!() }
    fn match_wildcard() {
        // Matching any value is the default behavior
        pub trait A {
            fn foo(&self, x: u32);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: u32);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any();

        mock.foo(42);
    }

    fn mock_struct() {
        struct GenericBean<T>(T);
        impl<T: Default> GenericBean<T> {
            pub fn eat(&self) -> T {
                T::default()
            }
        }
        struct GenericBeanMock<T> {
            e: Expectations,
            phantom: std::marker::PhantomData<*const T>
        }
        impl<T: 'static> GenericBeanMock<T> {
            pub fn new() -> Self {
                Self {
                    e: Expectations::new(),
                    phantom: std::marker::PhantomData
                }
            }
            pub fn eat(&self) -> T {
                self.e.was_called_returning::<(), T>("eat", ())
            }
            pub fn expect_eat(&mut self) -> Method<(), T> {
                self.e.expect::<(), T>("eat")
            }
        }

        let mut mock = Bean::new();
        mock.expect_eat().called_once();
        mock.eat();
    }

    fn mock_trait() {
        pub trait A {
            fn foo(&self, x: u32);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: u32);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any().with(42);

        mock.foo(42);
    }

    fn modules() { unimplemented!() }
    fn multi_trait() {
        // Simulacrum can mock multiple traits using mid-level macros
        pub trait A {
            fn foo(&self) -> u32;
        }
        pub trait B {
            fn bar(&self) -> u32;
        }

        create_mock_struct! {
            struct BMock: {
                expect_foo("foo") () => u32;
                expect_bar("bar") () => u32;
            }
        }
        impl A for BMock {
            fn foo(&self) -> u32 {
                was_called!(self, "foo", () -> u32)
            }
        }
        impl B for BMock {
            fn bar(&self) -> u32 {
                was_called!(self, "bar", () -> u32)
            }
         }

        let mut mock = BMock::new();
        mock.expect_foo().called_any().returning(|_| 42);
        mock.expect_bar().called_any().returning(|_| 99);

        assert_eq!(42, mock.foo());
        assert_eq!(99, mock.bar());
    }

    fn return_call_with_args() {
        pub trait A {
            fn foo(&self, x: u32) -> u32;
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: u32) -> u32;
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once().returning(|x| x + 1);

        assert_eq!(5, mock.foo(4));
    }

    fn return_constant() {
        unimplemented!()
    }

    fn return_default() {
        unimplemented!()
    }

    fn return_reference() {
        // I can't get this code to work.
        //pub trait A<'a> {
            //fn foo(&'a self) -> &'a u32;
        //}

        //struct AMock {
            //e: Expectations
        //}

        //impl<'a> AMock {
            //pub fn new() -> Self {
                //Self {
                    //e: Expectations::new()
                //}
            //}

            //pub fn expect_foo(&'a mut self) -> Method<(), &u32> {
                //self.e.expect::<(), &u32>("foo")
            //}
        //}

        //impl<'a> A<'a> for AMock {
            //fn foo(&'a self) -> &'a u32 {
                //self.e.was_called_returning::<(), &u32>("foo", ())
            //}
        //}

        //let mut mock = AMock::new();
        //mock.expect_foo().called_any().returning(|_| &5);

        //assert_eq!(5, *mock.foo());
        unimplemented!()
    }

    fn return_mutable_reference() { unimplemented!() }
    fn return_owned() {
        // Simulacrum returns the output of a `FnMut`, not an `FnOnce`, so it
        // can't return by move.
        // https://github.com/pcsm/simulacrum/issues/52
        unimplemented!()
    }

    fn return_panic() {
        unimplemented!()
    }

    fn return_parameters() {
        // Simulacrum can do this, but it needs unsafe code
        pub trait A {
            fn foo(&self, x: &mut u32);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: &mut u32);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any().modifying(|x: &mut *mut u32|
            unsafe {**x = 42}
        );

        let mut x = 0;
        mock.foo(&mut x);
        assert_eq!(42, x);
    }

    // https://github.com/pcsm/simulacrum/issues/56
    fn send() {
        unimplemented!()
    }

    // This doesn't work because Method contains a reference to the Expectations
    // object, and in our usage the reference must outlive the MutexGuard.  It
    // could be done with unsafe code, but why should the user need unsafe code
    // just to set an expectation?
    //lazy_static! {
        //static ref MOCK_STATIC: Mutex<Expectations>
            //= Mutex::new(Expectations::default());
    //}

    fn static_method() {
        //pub trait A {
            //fn bar() -> u32;
            //fn foo(&self, x: u32) -> u32;
        //}

        //create_mock_struct! {
            //struct AMock: {
                //expect_foo("foo") u32 => u32;
            //}
        //}
        //impl AMock {
            //fn expect_bar() -> Method<(), u32> {
                //MOCK_STATIC.lock().unwrap()
                    //.expect::<(), u32>("A_bar")
            //}
        //}
        //impl A for AMock {
            //fn foo(&self, x: u32) -> u32 {
                //was_called!(self, "foo", (x: u32) -> u32)
            //}
            //fn bar() -> u32 {
                //MOCK_STATIC.lock().unwrap()
                    //.was_called_returning::<(), u32>("A_bar", ())
            //}
         //}

        //let mut mock = AMock::new();
        //mock.expect_foo().called_once().returning(|_| 42);

        //assert_eq!(42, mock.foo(0));
        unimplemented!()
    }

    fn sequence() {
        // Simulacrum lacks this explicit functionality, but it can be
        // implemented using checkpoints, aka Eras.
        pub trait A {
            fn foo(&self);
            fn bar(&self);
            fn baz(&self);
            fn bang(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
                expect_bar("bar"):
                fn bar(&self);
                expect_baz("baz"):
                fn baz(&self);
                expect_bang("bang"):
                fn bang(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once();
        mock.then().expect_bar().called_once();
        mock.then().expect_baz().called_once();
        mock.then().expect_bang().called_once();

        mock.foo();
        mock.bar();
        mock.baz();
        mock.bang();
        print!("multi method ");
    }

    fn times_once() {
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once();

        mock.foo();
    }

    fn times_any() {
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any();

        mock.foo();
        mock.foo();
    }

    fn times_n() {
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_times(2);

        mock.foo();
        mock.foo();
    }

    fn times_never() {
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_never();
    }

    fn times_range() { unimplemented!() }

    fn version() {
        let ver = crate::built_info::DEPENDENCIES.iter()
            .find(|(name, _)| *name == "simulacrum")
            .unwrap()
            .1;
        print!("{} ", ver);
    }

    // While Simulacrum can mock generic traits and methods, the mock object is
    // concrete, not generic.  So where clauses don't really make sense.
    fn where_clause() { unimplemented!() }
}

test!{Simulacrum}

}
