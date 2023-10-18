# VPGAs

    An implementation of a VPGA in graph form

## TODO

    - Write LUT function
    - Write input block
    - Write output block
    - Struct and Impl for VPGA
    - Graph traversal function for VPGA

## Rules

    - No two way connections
    - No two pins are connected going both ways
    - Connections directions can be flipped

## The current flow problem

    - If all input connections to a pin are live then the pin is live
    - If one input connection to a pin are live then the pin is live
    - If no input connections to a pin are live then the pin is not live

// Algorithm:
// 1. Load input blocks with input in order
// 2. Iterate over each Input Block and for each pin in it start a breadth first update of neighbor pins
// 3. When breadth first update is done then iterate over the LUTs, take their updated input from the pins associated with them,
//    and perform a breadth first update from their output pin (ingore updating input pins)
// 4. When all of the breadth first updates are done from all LUTs then collect the output vector from the output pins
// 5. And use it to evaluate