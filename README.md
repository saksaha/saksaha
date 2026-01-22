# Saksaha: Anonymized Distributed Ledger & ZK-VM

**Saksaha** is a privacy-centric distributed ledger designed to execute
user-defined logic in a verifiable yet completely private manner. While previous
privacy blockchains focus solely on value transfer, Saksaha enables **arbitrary
state transitions** to be pseudonymized through a programmable **Virtual
Machine**.

## Motivation

Saksaha was born from the need to bridge the gap between programmable smart
contracts and absolute transactional privacy. The core technology is heavily
inspired by the foundational work in **Zerocash (2014)** by Eli Ben-Sasson et
al., focusing on decentralized anonymous computations.

## System Highlights

1. A custom P2P networking stack from the ground up, implementing a hybrid
   TCP/UDP architecture to ensure both reliable state synchronization and
   low-latency peer-to-peer communication..
2. A programmable execution layer powered by a WebAssembly (WASM) virtual
   machine, enabling deterministic and sandboxed execution of user-defined
   logic.
3. Saksaha transaction contains no metadata regarding the sender, receiver, or
   the amount being transferred. It instead consists of a zero-knowledge proof
   to allow users to claim ownership of an asset.
4. A unique data layer where users are allocated space without revealing their
   identity or the location of their stored state. The specific index or
   "address" of a user's data allocation is computed locally by the user

## References

- [Zerocash](https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=6956581):
  Decentralized Anonymous Payments from Bitcoin (Ben-Sasson et al., 2014)

## License

This project is licensed under the MIT License.
