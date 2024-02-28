This is a small rust program that reads the battery levels of each of a half of a split keyboard running ZMK.

Usage:

```
zmk_batt <MAC_ADDRESS>
```

To run locally, checkout this repo, and run:
```
cargo run -- <MAC_ADDRESS>
```

Or install the binary to your `~/.cargo/bin` directory using:
```
cargo install --path .
```

Requires your keyboard be paired to computer using BlueZ.
