pub struct Boks<T> {
    p: *mut T,
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
        }
    }
}

impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        // Safety: p was constructed from a Box in the first place, and has not been freed
        // otherwise since self still exists (ohterwise, drop could not be called).
        unsafe { Box::from_raw(self.p) };
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: is valid sine it was constructed from a valid T, and turned into a pointer
        // through Box with creates aligned pointers, and hasn't been freed, since self is alive.
        unsafe { &*self.p }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid sine it was constructed from a valid T, and turned into a pointer
        // through Box with creates aligned pointers, and hasn't been freed, since self is alive.
        // Also, since we have &mut self, no other mutable reference has been given out to p.
        unsafe { &mut *self.p }
    }
}

use std::fmt::Debug;

struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = 42;
    let b = Boks::ny(x);
    println!("{:?}", *b);

    let mut y = 42;
    let _b = Boks::ny(&mut y);
    // println!("{:?}", y); // Boks 实现的drop 编译器不能识别 &mut 和何时会被drop, 所以这里会报错

    let mut z = 42;
    let b = Boks::ny(Oisann(&mut z));
}
