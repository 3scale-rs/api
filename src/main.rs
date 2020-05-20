mod actions;
mod program;
mod readline;

fn main() {
    program::setup();
    let _ = program::write_info(&mut std::io::stdout());
    readline::repl(Some("./history.txt"));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
