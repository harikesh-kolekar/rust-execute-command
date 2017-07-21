# Run

A simple terminal menu to run commands.

# Create a `run.toml`:

```
[[item]]
code = "a"
desc = "ls current directory"
cmd = "ls -a"

[[item]]
code = "b"
desc = "See what is in run.toml"
cmd = "cat ./run.toml"
```

Then running `run` will show a menu:

<img src="./assets/menu.png" />