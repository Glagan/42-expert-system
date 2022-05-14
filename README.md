# expert-system

The goal of this project is to implement an [Expert system](https://en.wikipedia.org/wiki/Expert_system) that resolve a variant of [Propositional calculus](https://en.wikipedia.org/wiki/Propositional_calculus).

This program handle all of the project proposed bonus rules, ``OR`` and ``XOR`` in conclusions and *if and only if* (``<=>``) rules.

## Usage

```bash
USAGE:
    expert-system [OPTIONS] <file_paths>...

ARGS:
    <file_paths>...      Path to the input file(s)

OPTIONS:
    -h, --help           Print help information
    -i, --interactive    Update initial facts and queries in the shell
    -v, --visualize      Visualize the path to resolve a query
```

An input file path is always required.  
In interactive mode there is several commands to update the input or change the visualization:

```bash
e, exec	        Resolve the current queries
s, show	        Show the current rules, initial facts and queries
r, rule	        Add a rule
f, facts        Set the initial facts
?, queries	    Set the queries to resolve
n, next	        Go to the next file
v, visualize	Toggle visualization
h, help	        Print this help
q, quit	        Quit the program
```

## Resources

* https://en.wikipedia.org/wiki/Expert_system
    * https://en.wikipedia.org/wiki/Inference_engine
    * https://en.wikipedia.org/wiki/Backward_chaining
* https://en.wikipedia.org/wiki/Propositional_calculus
* Parser
	* https://doma.dev/blog/parsing-stuff-in-rust/
	* https://blog.logrocket.com/parsing-in-rust-with-nom/
* Inference Engine
	* https://en.wikipedia.org/wiki/SLD_resolution
