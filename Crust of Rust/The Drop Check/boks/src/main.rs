#![feature(dropck_eyepatch)]

use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

pub struct Boks<T> {
    p: NonNull<T>,      // NonNull<T> replace *mut T to make sure Boks is covariant
    _m: PhantomData<T>, // PhantomData<T> to make sure when drop the Boks drop checker will care about T
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Self {
            // use box to create data on heap, always return nonnull pointer
            // Safety: because Box never give out null pointer, so it is fine to use new_unchecked
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _m: PhantomData,
        }
    }
}

// Safety: there is no access the T inside the drop so it'is ok to use may_dangle attritube
unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // Safety: this is fine because the pointer came from a Box,
        // so it is safe to convert back to a box to drop it.
        let _ = unsafe { Box::from_raw(self.p.as_ptr()) };
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: pointer came from a Box<T>,
        // so it is fine to return a &T from the pointers
        unsafe { self.p.as_ref() }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: pointer came from a Box<T>,
        // so it is fine to return a &mut T from the pointer
        unsafe { self.p.as_mut() }
    }
}

use std::fmt::Debug;
pub struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        // Oisann drop access the T, so T msut outlive self
        // if T dropped before self, then this is a dangle pinter
        println!("inside oisann: {:?}", self.0)
    }
}

fn main() {
    let v = 42;
    let b = Boks::ny(v);
    println!("b: {}", &*b);

    // ALLOW: Drop not too restrictive
    // without may_dangle on the Drop this will compile fail
    // becaue the drop check thinks when drop(b),
    // may access the &mut y with the &mut y,
    // so the &mut y shoule live as long as the function end.
    // when comment out the Drop for the Boks, this will compile fine.
    // after add may_dangle attritube to the Drop, this will compile fine.
    // because there is no access &mut y inside b, and after println!("b: {}", *b); there is no access b
    // so the b can be drop after it.
    let mut y = 42;
    let b = Boks::ny(&mut y);
    println!("b: {}", *b);
    // drop(b);
    println!("y: {}", y);

    // COMPILE FAIL
    // // when drop the boks, it need to drop the T,
    // // but with the may_dangle attritube, drop checker will not check the T,
    // // so the type is vulnerable
    // let mut z = 42;
    // // this will compile fine, but this is wrong
    // let b = Boks::ny(Oisann(&mut z));
    // // Box will compile fail
    // // let b = Box::new(Oisann(&mut z));
    // println!("{:?}", z);
    // drop(z);
    // println!("z dropped");
    // // Oisann<&mut z> drop after this, drop Oisann will access z
    // // but z is already dropped, so the drop for the Oisann access a dangle pointer
    // // after add a field _m: PhantomData<T>, this will failed,
    // // PhantomData<T> tells the drop checker, the T is used, so make sure the T drop before self.
    // // if comment out Drop for the Oisann then this is fine,
    // // because b will drop before println!("{:?}", z);

    // because the field p: *mut T then T is invariant,
    // and the field _m: PhantomData<T> indicate that the T is covariant,
    // then there is a conflict, so the T in the Boks is invariant
    // but origin Box is covariant, so need to change the Boks variance
    // we could use NonNull to replace *mut

    // Variance demo for the Box
    // this code will work fine
    let s = String::from("hei");
    let mut box1 = Box::new(&*s);
    let box2: Box<&'static str> = Box::new("heisann");
    // is is allowed because Box<T> is covariant
    box1 = box2;

    // Variance demo for the Boks
    // with field p: *mut T this test will fail
    let s = String::from("hei");
    let mut box1 = Boks::ny(&*s);
    let box2: Boks<&'static str> = Boks::ny("heisann");
    // is is not allowed because Boks<T> is invariant
    // after use NonNull to replace *mut make Boks<T> covariant this works fine
    box1 = box2;

    // Demo for std::iter::Empty
    use std::iter::Empty;
    let mut x = 42;
    let mut empty_it: Empty<Oisann<&'static mut i32>> = Empty::default();
    // struct Empty<T>(PhantomData<T>)
    // let mut o: Option<Oisann<&'static mut i32>> = Some(Oisann(&mut x)); // <- this is wrong
    let mut o = Some(Oisann(&mut x));
    {
        o /* ...<&'a mut i32> */ = empty_it.next(); /* return ...<&'static mut i32> */
        // empty_it produce 'static lifetime get shorten
    }
    // &mut x drop before this
    drop(o);
    println!("{:?}", x);
    drop(x);
    // empty_it drop later is fine, because empty_it is never tied to the x
    // empty_it will always produce the Oisann<&'static mut i32> this is also never tied to the x
    // so this is fine
    let _ = empty_it.next();
    drop(empty_it);
}
