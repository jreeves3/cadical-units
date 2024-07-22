# cadical-units

'''
--unitcount         : number of units before exiting
--unitprint         : print units to stdout
--unitgap=<N>       : number of learned clauses between printed units
--unitgapgrow=<N>   : multiplier for gap after each printed unit
--unitstart=<N>     : number of learned clauses before unit priniting starts
'''

Units printed with format `c unit <unit> 0`

Must enable proof tracing with a proof file argument.

Example execution:
'''./cadical <form.cnf> proof.out --no-binary --unitcount=1 --unitprint'''

Modifications are made to the file proof.cpp in fucntion add_derived_unit_clause. This is where you can adjust what units are printed.

# Lrat-Cone
For both the Python and Rust implementation, command line flags are:
'''
--lrat <file_location> : the location of the lrat file to parse
--learned-gap <N>      : the minimum number of learned proof steps in between printed units
--unit-gap <N>         : the minimum number of units learned in between printed units 
--unit-count <N>       : the number of units to print 
--cone-size <N>        : the minimum size of the cone for a printed unit
'''

## Notes
When using the Rust tool, building with `--release` is strongly recommended. It can be used like `cargo run --release -- <args>`
as in `cargo run --release -- --lrat my_lrat.lrat --unit count 5`
