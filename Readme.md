# Flex Wrapper

Flex wrapper is a program wrapper that allows a cli program to be controlled via an http api.

---

## Usage

### With cargo:

```cargo run <command> [command-arguments]```

### Standalone:

```flex-wrapper <command> [command-arguments]```

---
## Routes

`/` Hello world

`/process/status` Get process status

`/process/start` Start the process if not running

`/process/kill` Kill the process if running