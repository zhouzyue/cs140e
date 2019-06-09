// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn default() -> Builder {
        Builder{
            string: None,
            number: None,
        }
    }

    fn string<T: ToString>(&mut self, s: T) -> &mut Self {
        self.string = Some(s.to_string());
        self
    }

    fn number(&mut self, num: usize) -> &mut Self {
        self.number = Some(num);
        self
    }
}

impl ToString for Builder {
    fn to_string(&self) -> String {
        match self {
            &Builder{string: ref s, number: None } => s.clone().unwrap_or("".to_string()),
            &Builder{string: Some(ref s), number: Some(n) } => s.clone() + " " + &n.to_string(),
            &Builder{string: None, number: Some(n) } => n.to_string(),
        }
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
