fn main() {
    let mut dummy = Foo::new().method_one(20).method_two(100);

    println!("{}", dummy.num);

    let another_dummy = dummy.method_one(10);

    //let dummy = Foo{num: 12};

    println!("{}", another_dummy.num);
    println!("{}", dummy.num);
}

#[derive(Clone)]
struct Foo {
    num: i32
}

impl Foo {
    fn new() -> Foo {
        Foo {
            num: 10
        }
    }

    fn method_one(&mut self, new_num: i32) -> Self {
        self.num = new_num;

        self.clone()
    }

    fn method_two(&mut self, new_num: i32) -> Self {
        self.num = new_num;

        self.clone()
    }
}
