use crate::*;

simple_day!(foo);

fn foo<UI: UIOutput<T>, T: Write>(_input: &str, _output: &mut UI) -> &'static str {
    "FOO"
}