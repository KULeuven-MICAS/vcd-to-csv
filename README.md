# vcd-to-csv

Generate csv files from a clocked vcd trace

Current limitations:
- Captures all signals in a fixed scope related to `gemmx`
- Assumes clock is `TOP.clk_i`
- requires file to be called `src/sim.vcd`
- outputs to a fixed `result.csv`
