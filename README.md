# *bob2020*

Example code and improvements automated via Criterion

## Quick start

Build and run benchmarks

```bash
$ cargo bench
```

Install criterion compare

```bash
$ cargo install critcmp
```

Summarise benchmark results

```bash
$ cargo critcmp
```

## Sample report

[Criterion Report](https://raw.githack.com/wayfair-tremor/bob2020/master/report/report/index.html)

## Known issues

Currently `rustc` optimises the `Static Borrow/Borrowed COW` test invalidating any comparison
for the `Static Borrow` micro-benchmark. This is a tooling issue and a drawback / reality with
micro-benchmarking with any tool / programming language; so we let the issue hang wet.
