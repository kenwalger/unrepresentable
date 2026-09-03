# Companion code for "Can Rust Make Unsafe AI Agent Actions Unrepresentable?"

This is a complete, runnable Cargo project. Its only job is to let you (a) confirm
the example behaves as the article claims, and (b) capture the *real* compiler
error for the article's "money shot" so it is authentic rather than hand-written.

## Confirm the legal flow compiles and runs

```
cargo run
```

Expected output:

```
persisted key=refund_policy value="Refunds under $100 do not require manager approval." authority=Policy
```

That is the propose -> evaluate -> persist path working.

## Capture the compiler error (the money shot)

1. Open `src/main.rs`.
2. At the bottom of `main`, uncomment the block marked `// MONEY SHOT`. It builds a
   fresh `ProposedWrite` and passes it straight to `persist()`.
3. Run:

   ```
   cargo build
   ```

4. Copy the actual `error[E0308]` block `rustc` prints, verbatim, including the
   `-->` location line and the `^^^^` span underlines. Paste it into the article
   where the `<!-- CAPTURE ... -->` placeholder sits.
5. Re-comment the block so repo HEAD builds cleanly again.

Do not edit the compiler output for style. Its authenticity is the point; a
Rust reader can tell hand-written rustc output at a glance.

## Note

`validate_authority` accepts `"security"` and `"policy"` and rejects everything
else, so you can also demonstrate the rejection path by changing the proposed
`authority` to something unrecognized and running `cargo run`.
