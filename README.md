My solutions to the programming puzzles in the [Advent of Code
2020](https://adventofcode.com/) written in Rust.

For other years see:

* [2020](https://github.com/wagdav/advent-of-code-2020)
* [2021](https://github.com/wagdav/advent-of-code-2021)
* [2022](https://github.com/wagdav/advent-of-code-2022)

# Build and run

Install the Nix package manager then

```
nix build
```

Then run the solutions

```
./result/bin/day01
./result/bin/day02
...
./result/bin/day25
```

# Develop

```
nix develop --command cargo {build,clippy,fmt,watch}
```
