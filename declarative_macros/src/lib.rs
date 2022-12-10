// 格式化打印宏输出
// cargo install cargo-expand
// cargo expand

#[macro_export]
macro_rules! avec {
    // () => {
    //     Vec::new()
    // };
    // ($($element:expr),*) => {{
    //     let mut vx = Vec::new();
    //     $(vx.push($element);)*
    //     vx
    // }};
    // $(,)? 支持数组最后一个元素带逗号","
    // ($($element:expr),* $(,)?) => {{
    //     let mut vx = Vec::new();
    //     $(vx.push($element);)*
    //     vx
    // }};

    ($($element:expr),*) => {{
        #[allow(unused_mut)]
        let mut vx = Vec::new();
        $(vx.push($element);)*
        vx
    }};

    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};

    // [N;n]
    // ($element:expr; $count:expr) => {{
    //     let mut vx = Vec::new();
    //     let x = $element;
    //     for _ in 0..$count {
    //         vx.push(x.clone());
    //     }
    //     vx
    // }}
    ($element:expr; $count:expr) => {{
        let mut vx = Vec::new();
        // ::std 表示 std必须为一个crate
        // vx.extend(::std::iter::repeat($element).take(count));
        vx.resize($count, $element);
        vx
    }}
}

#[test]
fn empty_avec() {
    let x: Vec<u32> = avec!();
    assert!(x.is_empty())
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 47];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 47);
}

// 允许最后一个元素有逗号 ","
#[test]
fn ttailing() {
    let x: Vec<u32> = avec![42, 47,];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 47);
}

#[test]
fn array() {
    let x: Vec<i32> = avec![5;5];
    assert_eq!(x.len(), 5);
    assert_eq!(x, vec![5, 5, 5, 5, 5])
}

#[test]
fn array_nonliteral() {
    let mut y = Some(5);
    let x: Vec<i32> = avec![y.take().unwrap();5];
    assert_eq!(x.len(), 5);
    assert_eq!(x, vec![5, 5, 5, 5, 5])
}

/// ```compile_fail
/// let x: Vec<i32> = declarative_macros::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;

trait MaxValue {
    fn max_value() -> Self;
}

#[macro_export]
macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

#[test]
fn max_value() {
    max_impl!(i32);
    max_impl!(u32);
    max_impl!(i64);
    max_impl!(u64);
}
