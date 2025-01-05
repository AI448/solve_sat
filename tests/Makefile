
SOLVE_SAT=../target/debug/solve_sat

.PHONY: ${SOLVE_SAT}

${SOLVE_SAT}:
	cargo build --bin=$(shell basename ${SOLVE_SAT})

test_solve_sat: ${SOLVE_SAT} scripts/test_solve_sat.py
	python scripts/test_solve_sat.py ./instance
