# Seam SDK

Seam SDK is a minimal runtime for bounded, task-scoped AI workers. It runs one assigned task with a step budget, a bounded observation window, local workbench operations, and a terminal report. The worker has no durable or global memory by default.

Users bring their own inference infrastructure, gateway, external operations, and agent platform integration. The SDK does not provide an AI gateway, model routing, a capability decision engine, a tool gateway, or durable global memory.

```text
Seam SDK ── Reasoner + ExternalRuntime ──> existing infrastructure

Seam SDK ── optional adapter ──> Seam Engine ──> existing gateway
```

`Reasoner` answers “what should this worker do next?” `ExternalRuntime` performs an intent outside the local workbench. They may use the same downstream infrastructure, but remain separate worker concerns. Neither port has a built-in model, provider, tool, or capability catalog. `Seam-SDK` has no dependency on `Seam-Engine`.

## Run the standalone fixture

```sh
cargo test --all-targets
cargo run --example standalone
```

The example uses a deterministic reasoner and external runtime. It needs no gateway, credentials, or Engine deployment. A real deployment can implement these two traits against its existing infrastructure.

## Local boundary

`FsProcessWorkbench` enforces workspace-relative paths, checks the final symlink target, intersects per-task local authority with deployment ceilings, clears ambient child environment variables, and caps observations and returned output. Process execution has a timeout. The caller assigns `WorkerId`; the worker does not mint an identity or external grant.

The workbench is not an operating-system sandbox. A deployment that permits local process execution must supply real isolation and network controls around it. External authority must be checked by the `ExternalRuntime` implementation against trusted grants; a worker's ticket is not evidence of external permission.

## Optional integration

The [combined example](https://github.com/Judge-M/Seam/tree/main/examples/combined) implements both SDK ports through Seam Engine and a fixture unified gateway. It lives outside both implementation crates so they stay independently usable.

Licensed under Apache-2.0. See [LICENSE](LICENSE).
