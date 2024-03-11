# VPGAs

    An implementation of a VPGA in graph form.

    A VPGA is a Virtual Programmable Gate Array. Simular to a physical FGPA it is a virtual form of a chip that maps binary data vectors together by the function it represents.
    VGPAs are made up of logical units that are connected by virtual wires that are unidirectional.

    The units of a VPGA in this implementation are:
    
        - Pin - A virtual 'pin' that has a state of either on or off
            - Acts as an anchor for connections 
                - Which are virtual wires
        - LUT - Look Up Table
            - Operates exactly like a physical look up table in a digital chip. It computes binary functions represented by truth tables
        - Input Block
            - Takes binary data as input and loads it into the pins inside of it for later use
        - Output Block
            - Has pins inside of it that are loaded through the simulation. Binary data is read off of these pins as the output of the VPGA
        - Switch Box
            - A virtual collection of pins
            - Used to add the ability to make more complex functions by wiring more components together
        - Encoding
            - A binary representation of the configuration of a VPGA
                - Represents connectivity between all pins of the VPGA
        - VPGA Spec
            - A specification for a given VPGA
            - Determines the number of LUTS, their narrowing factor, pin configuration, and other things

    A VPGA is operated by the following steps:

        1. Binary data is loaded into the input blocks
        2. Breadth first propogation happens for the whole chip starting from every input pin
        3. Every LUT is operated in sequence
        4. Breadth first propogation happens for the whole chip starting from every output pin for each LUT
        5. Output is read from the output pins

    The goal is then to search for an encoding that will generate a 1 to 1 mapping between the binary data that is wished to be mapped together.

    I have currently only implemented Tabu Search as the search algorithm but others may be implemented in the future.

## Initial Results

    - Initial results show that simple functions can be mapped using simple VPGA specifications.
    - For any large designs the simulation is defeated by the complexity of running Breadth First Search too many times

## Future Work

    - For this simulation to become viable it needs to be at least multithreaded
    - Possible port to the GPU


## Rules

    - No two way connections
    - No two pins are connected going both ways
    - Connections directions can be flipped