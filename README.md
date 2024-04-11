# cadical-units

'''
--unitcount           : number of units before exiting
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