use std::time;

use crate::{
    analyze::{Analyze, AnalyzeResult},
    binary_clause_theory::BinaryClauseTheory,
    calculate_lbd::CalculatePLBD,
    clause_theory::ClauseTheory,
    cnf::CNF,
    engine::{ExplainKey, make_constraint},
    plbd_watcher::PLBDWatcher,
    pricer::Pricer,
};
use cdcl_engine::{
    Boolean, CoreEngine, EngineAddConstraintTrait, EngineTrait, Literal, OuterEngine, PropagationResult, Reason,
};

pub fn solve(cnf: &CNF) -> bool {
    let start_time = time::Instant::now();

    let mut engine = OuterEngine::new(
        ClauseTheory::new(1e4),
        OuterEngine::new(BinaryClauseTheory::default(), CoreEngine::<ExplainKey>::default()),
    );
    let mut pricer = Pricer::new(1e2);
    let mut analyze = Analyze::default();
    let calculate_lbd = CalculatePLBD::default();
    let mut plbd_watcher = PLBDWatcher::new(1e4, 1e2);

    let number_of_variables = cnf.clauses.iter().map(|c| c.iter().map(|l| l.index()).max().unwrap()).max().unwrap() + 1;
    for _ in 0..number_of_variables {
        engine.add_variable(Boolean::FALSE);
        pricer.add_variable(0.0, false);
    }

    for clause in cnf.clauses.iter() {
        let constraint = make_constraint(clause.iter().cloned());
        let result = engine.add_constraint(constraint.into(), false);
        if result.is_conflict() {
            println!("UNSATISFIABLE,{},{}", 1, start_time.elapsed().as_secs_f64());
            return false;
        }
    }

    let mut restart_count: usize = 0;
    let mut conflict_count: usize = 0;
    macro_rules! print_progress {
        () => {
            let summary = engine.summary();
            eprintln!(
                "restart={}, conflict={}, plbd={}, fixed={}, binary_clauses={}, learnt_binary_clauses={}, clauses={}, learnt_clauses={}",
                restart_count,
                conflict_count,
                plbd_watcher.long_term_average(),
                summary.1.1.number_of_fixed_variables,
                summary.1.0.number_of_binary_clauses, summary.1.0.number_of_learnt_binary_clauses,
                summary.0.number_of_clauses, summary.0.number_of_learnt_clauses,
            );
        }
    }

    let mut conflict_count_at_previous_restart = 0;

    let mut propagation_result = PropagationResult::Noconflict;
    loop {
        if start_time.elapsed() > time::Duration::from_secs(60) {
            print_progress!();
            println!("INDEFINITE,{},{}", conflict_count, start_time.elapsed().as_secs_f64());
            return false;
        }
        if let PropagationResult::Conflict { explain_key } = propagation_result {
            conflict_count += 1;
            let analyze_result = analyze.analyze(explain_key, &engine);
            match analyze_result {
                AnalyzeResult::Unsatisfiable => {
                    print_progress!();
                    println!("UNSATISFIABLE,{},{}", conflict_count, start_time.elapsed().as_secs_f64());
                    return false;
                }
                AnalyzeResult::Backjumpable { backjump_decision_level, learnt_clause, related_variables } => {
                    let plbd = calculate_lbd.calculate(learnt_clause.iter(), &engine);
                    plbd_watcher.add(plbd);

                    pricer.increase_price(related_variables);
                    {
                        let unassigned_literals = engine.backjump(backjump_decision_level);
                        for unassigned_literal in unassigned_literals {
                            pricer.set_to_unassigned(unassigned_literal.index());
                        }
                    }
                    propagation_result = engine.add_constraint(learnt_clause.into(), true);
                }
            }
        } else if plbd_watcher.ratio() > 1.2 || conflict_count >= conflict_count_at_previous_restart + 10000 {
            restart_count += 1;
            if engine.current_decision_level() != 0 {
                let unassigned_literals = engine.backjump(0);
                for unassigned_literal in unassigned_literals {
                    pricer.set_to_unassigned(unassigned_literal.index());
                }
            }
            engine.reduce_constraints();
            plbd_watcher.clear_short_term_average();
            conflict_count_at_previous_restart = conflict_count;
            print_progress!();
        } else {
            let decision_index = {
                let mut index;
                loop {
                    index = pricer.peek();
                    if index.is_none() {
                        print_progress!();
                        println!("SATISFIABLE,{},{}", conflict_count, start_time.elapsed().as_secs_f64());
                        return false;
                    }
                    pricer.set_to_assigned(index.unwrap());
                    if !engine.is_assigned(index.unwrap()) {
                        break;
                    }
                }
                index.unwrap()
            };
            let decision_value = engine.get_value(decision_index);
            propagation_result = engine.assign(Literal::new(decision_index, decision_value), Reason::Decision);
        }
    }
}
