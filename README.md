# vcd-to-csv

Generate csv files from a clocked vcd trace

Current limitations:
- Captures all signals in a fixed scope related to `gemmx`
- Assumes clock is `TOP.clk_i`
- requires file to be called `sim.vcd`
- outputs to a fixed `sim.csv`
- Does not support vector values, these are all set to 0, only scalar values are supported for now.
