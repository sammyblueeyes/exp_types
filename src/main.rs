use std::fmt::Debug;

trait A {
    type Error: Debug;

    fn do_a(&self) -> Result<(), Self::Error>;
}

trait B {
    type Error: Debug;

    fn do_b(&self) -> Result<(), Self::Error>;
}

#[derive(thiserror::Error)]
enum GenError<T>
where
    T: A + B,
{
    GenError,
    AError(#[from] <T as A>::Error),
    BError(#[from] <T as B>::Error),
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
        Ok(self.gen_val.do_a()?)
    }

    fn call_b(&self) -> Result<(), GenError<T>> {
        Ok(self.gen_val.do_b()?)
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

#[derive(Debug)]
struct Ter {}

#[derive(Debug)]
enum TerErrors {
    AFailedForT,
    BFailedForT,
}

impl A for Ter {
    type Error = TerErrors;

    fn do_a(&self) -> Result<(), Self::Error> {
        Err(TerErrors::AFailedForT)
    }
}

impl B for Ter {
    type Error = TerErrors;

    fn do_b(&self) -> Result<(), Self::Error> {
        Err(TerErrors::BFailedForT)
    }
}

fn main() {
    println!("Hello, world!");

    let mut t = Tee {};
    let v = GenVal { gen_val: &mut t };
    println!("{:?}", v.call_a());
    println!("{:?}", v.call_b());
    println!("{:?}", v.call());

    let mut t = Ter {};
    let v = GenVal { gen_val: &mut t };
    println!("{:?}", v.call_a());
    println!("{:?}", v.call_b());
    println!("{:?}", v.call());
}
