use std::{borrow::Cow, marker::PhantomData, ops::Deref};

use crate::Node;

pub trait Lens: 'static {

    type Source: Clone + Node;
    type Target: Clone;

    fn view<'a>(&self, data: Cow<'a, Self::Source>) -> Cow<'a, Self::Target>;
}


/// Helpers for manipulating `Lens`es
pub trait LensExt: Lens {

    /// Compose a `Lens<Source = A, Target = B>` with a `Lens<Source = B, Target = C>` to produce a `Lens<Source = A, Target = C>`
    fn then<Other>(self, other: Other) -> Then<Self, Other>
    where
        Other: Lens + Sized,
        Self: Sized,
    {
        Then::new(self, other)
    }
}

// Implement LensExt for all types which implement Lens
impl<T: Lens> LensExt for T {

}

/// `Lens` composed of two lenses joined together
#[derive(Debug, Copy)]
pub struct Then<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left, Right> Then<Left, Right> {
    pub fn new(left: Left, right: Right) -> Self
    where
        Left: Lens,
        Right: Lens,
    {
        Self {
            left,
            right,
        }
    }
}

impl<Left, Right> Lens for Then<Left, Right>
where
    Left: Lens,
    Right: Lens<Source = <Left as Lens>::Target>,
{

    type Source = <Left as Lens>::Source;
    type Target = <Right as Lens>::Target;

    fn view<'a>(&self, data: Cow<'a,Self::Source>) -> Cow<'a, Self::Target> {
        self.right.view(self.left.view(data))
    }
}

impl<T: Clone, U: Clone> Clone for Then<T, U> {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}


pub struct And<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left, Right> Lens for And<Left, Right> 
where 
    Left: Lens,
    Right: Lens<Source = <Left as Lens>::Source>,
{
    type Source = <Left as Lens>::Source;
    type Target = (Left::Target, Right::Target);

    fn view<'a>(&self, data: Cow<'a,Self::Source>) -> Cow<'a, Self::Target> {
        Cow::Owned((self.left.view(data.clone()).into_owned(), self.right.view(data.clone()).into_owned()))
    }
}
