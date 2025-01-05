use cdcl_engine::{Boolean, Literal};
use utility::Array;

use crate::cnf::CNF;

fn parse_line(line: &str) -> Array<u32, Literal> {
    let mut clause = Array::default();
    for field in line.split_whitespace() {
        let i = field.parse::<i64>().unwrap();
        if i == 0 {
            break;
        }
        let index = (i.abs() - 1) as u32;
        let value = if i > 0 { Boolean::TRUE } else { Boolean::FALSE };
        clause.push(Literal::new(index, value));
    }
    assert!(!clause.is_empty());
    return clause;
}

pub fn read_cnf(reader: impl std::io::BufRead) -> CNF {
    let mut cnf = CNF::default();
    for read_result in reader.lines() {
        let line = read_result.unwrap();
        if line.is_empty() || line.starts_with('c') || line.starts_with('p') {
            continue;
        }
        cnf.clauses.push(parse_line(&line));
    }
    return cnf;
}
