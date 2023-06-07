# Must Done
An experimental Future that is guaranteed to be done.

## What It Does
In some cases, we need to ensure that the Future is executed and terminated, especially those Futures that have side effects when canceled.

For example, under io_uring, because we cannot ensure that the Future is executed, we have to pass the buffer ownership on the IO interface, which makes the interface very cumbersome to use.

Using this Future wrapper, users can be sure that the Future will be terminated at compile time.

## How It Works
Inspired by dtolnay's no-panic, I define a Future Wrapper and placed a Guard in it.

If the Future can not be proven terminated, then the Guard will be dropped, and the drop method of the Guard needs to link to an external library that does not exist, which will trigger an error during the link period; and if the Future must be terminated, the drop implementation will not be used due to dead code elimination.

## Some Notes
1. In order to ensure that dead code elimination can work correctly, a higher opt-level needs to be configured.
2. When the app panic, the future will be dropped even itself can definitely terminate. So users may set `panic = "abort"` to avoid future be dropped due to panic possibility.

```toml
[profile.dev]
opt-level = 3
panic = "abort"
```
