# VPGAs

    An implementation of a VPGA in graph form

## TODO

    - LUT operate
    - VPGA operate
    - Simulation
    - Tabu search
    - Concurrency

## Rules

    - No two way connections
    - No two pins are connected going both ways
    - Connections directions can be flipped

## The current flow problem

    - If all input connections to a pin are live then the pin is live
    - If one input connection to a pin are live then the pin is live
    - If no input connections to a pin are live then the pin is not live