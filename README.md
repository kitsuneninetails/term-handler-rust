# Term Handler

Handles waiting for a termination or interrupt signal with a loop.  The loop will exit
once the SIGTERM/SIGINT is sent (usually via Ctrl-C).

Usage:

```rust
    print!("Waiting for TERM signal");
    wait_for_term();
    print!("Term signal received, quitting!");
```
