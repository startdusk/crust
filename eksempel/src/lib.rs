pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn foo() {
    strlen("hello world"); // &'static str
    strlen(String::from("hei verdon")); // String
}

// 编译器会对strlen生成如下函数(名字不是这个)
pub fn strlen_refstr(s: &str) -> usize {
    s.len()
}
pub fn strlen_string(s: String) -> usize {
    s.len()
}

pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
    if b {
        Some(f())
    } else {
        None
    }
}

pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self)
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self)
    }
}

pub fn bar(s: &[&dyn Hei]) {
    for i in s {
        // dyn Hei, vtable:
        //
        //  struct HeiVtable {
        //    hei: &mut Fn(*mut ()),
        //  }
        //
        // &str -> &dyn Hei
        // 1.pointer to the str
        // 2.&HeiVtable {
        //     hei: &<str as Hei>::hei
        // }
        i.hei()
        // i.vtable.hei(s.pointer)
    }
}

pub fn call_bar() {
    // trait object
    // 由于实现trait的对象大小都不一致，
    // 所以必须使用指针，因为能保证size都是一致的
    bar(&[&"j", &"hello", &String::from("fdsa")])
}
