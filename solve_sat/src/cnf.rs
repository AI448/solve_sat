use cdcl_engine::Literal;
use utility::Array;

#[derive(Default)]
pub struct CNF {
    pub clauses: Array<u32, Array<u32, Literal>>,
}
