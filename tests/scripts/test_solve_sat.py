import os
import sys
import datetime
from pathlib import Path
from subprocess import Popen

from enumerate_files import enumerate_files


solve_sat = Path(__file__).parent / ".." / ".." / "target" / "debug" / "solve_sat"


def test_solve_sat(target_dirpath: Path):

    result_dirpath = Path("result") / datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    os.makedirs(result_dirpath)

    for target_filepath in enumerate_files(target_dirpath, ".cnf"):
        print("{},".format(target_filepath.name), flush=True, end="")
        logfile = open(result_dirpath / target_filepath.name.replace(".cnf", ".txt"), "w")
        process = Popen([solve_sat], stdin=open(target_filepath), stderr=logfile)
        code = process.wait()
        logfile.close()

if __name__ == "__main__":
    test_solve_sat(Path(sys.argv[1]))
