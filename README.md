# LogChop

_It's better than bad, it's good!_

Log your `Result` and `Option` chains with inline methods.

Turn

```rust
match maybe_something {
    Some(x) => Some(x),
    None => {
        debug!("Nothing found!");
        None
    }
}.
```

into

```rust
maybe_something.debug_none("Nothing found!")
```

This becomes handy when you start chaining from results

```rust
use logchop::*;

parse_id_string(id_str)
    .trace_ok("Found id")
    .debug_err("Couldn't parse ID")
    .map_or_else(|id| get_widget_by_id(id), |_| get_default_widget())
    .info_ok_format(|widget| format!("Found widget: {}", widget.name))
```

```
# Sucessful parse result
[trace] Found id: 12
[info ] Found widget: chainsaw

# Error parsing result
[debug] Couldn't parse ID: Invalid format
[info ] Found widget: concilation prize
```

Say goodbye to unnecessary blocks!

## Installation and Usage

Add the following to your `Cargo.toml`:

```toml
logchop = "0.1"
```

and then import traits: `use logchop::*`
