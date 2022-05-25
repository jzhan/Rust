trait Foo {
    fn f_message(&self) {
        println!("This is Foo trait");
    }
}

trait Bar {
    fn b_message(&self) {
        println!("This is Bar trait");
    }
}

struct Dummy {
    _name: String
}

impl Foo for Dummy {}
impl Bar for Dummy {}

fn bazz(m: &(impl Foo + Bar)) -> &(impl Foo + Bar) {
    m
}

fn main() {
    let dum = Dummy {_name: "Dum dum".to_string()};

    let s = bazz(&dum);

    s.f_message();
    s.b_message();
}
