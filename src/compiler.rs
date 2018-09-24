use structured::*;

pub trait Compiler {
    fn visit_limit_clause(&self) {}

    fn visit_select(&self, _select: &Select) -> String {
        String::from("SELECT ")
    }
}
