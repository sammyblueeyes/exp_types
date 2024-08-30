use std::convert::From;
use std::fmt::Debug;

trait A {
    type Error: Debug;

    fn do_a(&self) -> Result<(), Self::Error>;
}

trait B {
    type Error: Debug;

    fn do_b(&self) -> Result<(), Self::Error>;
}

#[derive(Debug)]
enum GenError<T>
where
    T: A + B,
{
    GenError,
    AError(<T as A>::Error),
    BError(<T as B>::Error),
}

// Reproduce the failure:
// error[E0119]: conflicting implementations of trait `From<GenError<_>>` for type `GenError<_>`
impl<T> From<<T as A>::Error> for GenError<T> {
    fn from(item: <T as A>::Error) -> Self {
        Self::AError(item)
    }
}

struct GenVal<'a, T>
where
    T: A + B,
{
    gen_val: &'a mut T,
}

// impl of GenVal for a generic type `T`
impl<'a, T> GenVal<'a, T>
where
    T: A + B,
{
    fn call_a(&self) -> Result<(), GenError<T>> {
        match self.gen_val.do_a() {
            Ok(_) => Ok(()),
            Err(e) => Err(GenError::AError(e)),
        }
    }

    fn call_b(&self) -> Result<(), GenError<T>> {
        match self.gen_val.do_b() {
            Ok(_) => Ok(()),
            Err(e) => Err(GenError::BError(e)),
        }
    }
    fn call(&self) -> Result<(), GenError<T>> {
        Err(GenError::GenError)
    }
}

#[derive(Debug)]
struct Tee {}

#[derive(Debug)]
enum TeeErrors {
    AFailedForT,
    BFailedForT,
}

impl A for Tee {
    type Error = TeeErrors;

    fn do_a(&self) -> Result<(), Self::Error> {
        Err(TeeErrors::AFailedForT)
    }
}

impl B for Tee {
    type Error = TeeErrors;

    fn do_b(&self) -> Result<(), Self::Error> {
        Err(TeeErrors::BFailedForT)
    }
}

fn main() {
    println!("Hello, world!");

    let mut t = Tee {};

    let v = GenVal { gen_val: &mut t };
    println!("{:?}", v.call_a());
    println!("{:?}", v.call_b());
    println!("{:?}", v.call());
}
