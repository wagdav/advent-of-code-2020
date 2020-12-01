Advent of Code 2020

# Build

Install the Nix package manager then

```
nix build
```

Then run the solutions

```
./result/bin/day01
```

# Develop

```
nix develop --command cargo build
nix develop --command cargo clippy
nix develop --command cargo fmt
```
