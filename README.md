# cargo-ease
Cargo subcommand to generate EE project template

## build

```bash
$ cargo build
```

## run

**Inputs provided as args**

```bash
$ cargo run ease -n foobar
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/cargo-ease ease -n foobar`
⛏   Creating project called `foobar`...
🦄   Boom! New project created ~/cargo-ease/foobar       
```

**Inputs provided via prompt**

```bash
$ cargo run ease
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/cargo-ease ease`
❔  Project Name: foobar
⛏   Creating project called `foobar`...
🦄   Boom! New project created ~/cargo-ease/foobar            
```
