fn main() {
    println!("Hello, world!");

    let x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x)); // 0

    baz(x); // 8

    baz(bar::<u32>); // 8
    baz(bar::<i32>); // 8
}

fn bar<T>() {}

fn baz(f: fn()) {
    println!("{}", std::mem::size_of_val(&f))
}

// impl<F> FnOnce for F
// where
//     F: Fn(),
// {
//     fn call(self) {
//         Fn::call(&self)
//     }
// }

// impl<F> FnOnce for F
// where
//     F: FnMut(),
// {
//     fn call(mut self) {
//         Fn::call(&mut self)
//     }
// }

// impl<F> FnMut for F
// where
//     F: Fn(),
// {
//     fn call(&mut self) {
//         Fn::call(&*self)
//     }
// }
