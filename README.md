# Private Transaction Program

<a href="https://github.com/zz-sol/private-tx-program/actions/workflows/main.yml"><img src="https://img.shields.io/github/actions/workflow/status/zz-sol/private-tx-program/main.yml?logo=GitHub" /></a>

Shielded pools on Solana: an on-chain pool program gated by zero-knowledge
proofs, the circuits those proofs are made for, and the tests that run one
against the other.

This repository is a skeleton. Every directory below is a placeholder that
the implementation will be migrated into.

## Layout

| Path | What |
| --- | --- |
| [`program`](./program) | The pool program |
| [`e2e`](./e2e) | End-to-end tests of the circuits against the built program |
| [`circuits`](./circuits) | Go module: the circuits, the proving client and the fixture generator |
| [`fixtures`](./fixtures) | Generated keys, proofs and public witnesses that `e2e` consumes |

## Program

You can build / test / format the pool program in `program` using the
following commands:

```sh
make build-sbf-program
make test-program
make format-check-program
make clippy-program
```

The same targets exist for `e2e`, for example `make test-e2e`. The targets
take a nightly toolchain and a Solana CLI version, pinned in
[`Makefile`](./Makefile) and [`Cargo.toml`](./Cargo.toml).
