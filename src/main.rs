trait Visitor {
    // 想传入闭包,又是安全的 trait object 就要使用& 才行.
    // 这里可以用 Box 包住 trait object 这样就是对象安全的 trait object 了.
    // https://stackoverflow.com/questions/30055356/the-trait-a-is-not-implemented-for-the-type-a
    // 这里 用& 引用trait object 相当于 Box 包裹的.一种指针类型.
    fn visit(&self, f: &dyn Fn());
}

// 这里必须使用 Box 包裹 trait object, 否则会提示是 unsized 类型.
type VisitorList = Vec<Box<dyn Visitor>>;
impl Visitor for VisitorList {
    fn visit(&self, f: &dyn Fn()) {
        for i in self.iter() {
            i.visit(&|| {
                println!("In VisitorList before fn");
                f(); println!("In VisitorList before fn");
            })
        }
    }
}

struct Visitor1;

impl Visitor for Visitor1 {
    fn visit(&self, f: &dyn Fn()) {
        println!("In Visitor1 before fn");
        f();
        println!("In Visitor1 before fn");
    }
}

struct Visitor2 {
    visitor: Box<dyn Visitor>,
}

impl Visitor for Visitor2 {
    fn visit(&self, f: &dyn Fn()) {
        self.visitor.visit(&|| {
            println!("In Visitor2 before fn");
            f();
            println!("In Visitor2 before fn");
        });
    }
}

struct Visitor3 {
    visitor: Box<dyn Visitor>,
}

impl Visitor for Visitor3 {
    fn visit(&self, f: &dyn Fn()) {
        self.visitor.visit(&|| {
            println!("In Visitor3 before fn");
            f();
            println!("In Visitor3 before fn");
        });
    }
}

fn main() {
    let mut visitor: Box<dyn Visitor>;
    let visitors: Vec<Box<dyn Visitor>>;
    visitor = Box::new(Visitor1 {}) as Box<dyn Visitor>;
    visitors = vec![visitor];
    visitor = Box::new(Visitor2 {
        visitor: Box::new(visitors) as Box<dyn Visitor>,
    });
    visitor = Box::new(Visitor3 {
        visitor: visitor as Box<dyn Visitor>,
    });
    visitor.visit(&|| println!("yes"))
}

// 用 rust 来写 golang 那一套 很舒服,就是用组合的方式.不像
// Java强制使用面向对象范式被胁迫的感觉难受.
