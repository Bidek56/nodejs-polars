# Column mapping expressions

This document describes two approaches to adding a "column map" expression to nodejs-polars:

1) mapDict(mapping) — native, fast
   - Usage:
     ```ts
     df.withColumns([pl.col("a").mapDict({ B: "OtherB" }).alias("otherA")]);
     ```
   - Implementation: the JS mapping object is converted into a Rust HashMap and an Expr
     is created that performs a vectorized lookup/replace at evaluation time. This runs
     entirely in Rust/Polars and is the recommended approach for literal mappings.

2) map(fn) — JS callback, element-wise (advanced, slower)
   - Usage:
     ```ts
     const mapping = { B: "OtherB" };
     df.withColumns([
       pl.col("a").map(v => mapping[v] ?? v).alias("otherA")
     ]);
     ```
   - Implementation notes:
     - A ThreadsafeFunction (or equivalent) is created in the native binding when JS calls `.map(fn)`.
     - When the Expr is evaluated, the native closure serializes a chunk of the column (e.g., Vec<Option<T>>),
       calls the JS callback synchronously, and expects a mapped Vec<Option<T>> back.
     - The closure then reconstructs a Series from the mapped values and returns it to Polars.
   - Caveats:
     - Significant overhead due to serialization and cross-language calls.
     - The mapping callback runs synchronously during expression evaluation; heavy JS work will block.
     - Nulls and dtype conversions must be handled carefully.
     - This approach is suitable for prototyping or when the mapping can't be expressed natively.

Design recommendations
- Implement mapDict first:
  - It covers most use-cases (literal lookups), is fast, and avoids complex threading/serialization.
  - Add JS tests demonstrating behavior with strings, nulls, and missing keys.
- If map(fn) is required:
  - Prototype threadsafe function usage in a small example outside Polars first.
  - Design a clear serialization contract (e.g., string columns -> Vec<Option<String>>, numeric columns -> Vec<Option<f64>>).
  - Document the performance implications clearly.

Testing
- Add unit tests for mapDict: basic mapping, null handling, and unchanged fallback behavior.
- Add integration tests for map(fn) if implemented; include a basic example and edge cases.

TODOs for maintainers
- Wire the native functions `add_map_dict_expr` and `add_map_callback_expr` in the C/++/napi/neon layer.
- Add TypeScript type definitions and typings for the new methods.
- Add CI tests that run the JS unit tests for the new behavior.
