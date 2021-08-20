#![feature(generic_associated_types)]

use std::marker::PhantomData;

use crate::Node;

pub trait Lens: 'static {

    type Source: Node;
    type Target<'a>;

    fn view<'a>(&self, data: &'a Self::Source) -> Self::Target<'a>;
}


/// Helpers for manipulating `Lens`es
pub trait LensExt {

    /// Compose a `Lens<Source = A, Target = B>` with a `Lens<Source = B, Target = C>` to produce a `Lens<Source = A, Target = C>`
    fn then<'a, Other>(self, other: Other) -> Then<Self, Other>
    where
        Other: Lens + Sized,
        Self: Lens + Sized,
    {
        Then::new(self, other)
    }

    fn and<'a, Other>(self, other: Other) -> And<Self, Other>
    where
        Other: Lens,
        Self: Lens + Sized,
    {
        And::new(self, other)
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
    for<'a> Left: Lens<Target<'a> = &'a <Right as Lens>::Source>,
    Right: Lens,
{

    type Source = <Left as Lens>::Source;
    type Target<'a> = <Right as Lens>::Target<'a>;

    fn view<'a>(&self, data: &'a Self::Source) -> Self::Target<'a> {
        self.right.view(&self.left.view(data))
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

impl<Left, Right> And<Left, Right> {
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

impl<Left, Right> Lens for And<Left, Right> 
where
    Left: Lens,
    Right: Lens<Source = <Left as Lens>::Source>,
{
    type Source = <Left as Lens>::Source;
    type Target<'a> = (Left::Target<'a>, Right::Target<'a>);

    fn view<'a>(&self, data: &'a Self::Source) -> Self::Target<'a> {
        (self.left.view(data), self.right.view(data))
    }
} 