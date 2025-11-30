// TypeScript wrapper additions for column map expressions.
// Adjust imports/exports to match the project's module layout.
// This file exposes two convenience helpers on Expr:
//   - mapDict(mapping: Record<string,string>): Expr  (native, fast)
//   - map(fn: (v:any) => any): Expr                  (JS callback - skeleton)
//
// NOTE: native binding function names below (native.add_map_dict_expr, native.add_map_callback_expr)
// are placeholders. The native binding implementation must expose these functions
// and follow the contract: accept an Expr handle and return a new Expr handle.

import native from "./native"; // adjust to actual native binding module
import { Expr as NativeExpr } from "./expr_native"; // adjust path to the repo's Expr type

declare module "./expr_native" {
  interface Expr {
    mapDict(mapping: Record<string, string>): Expr;
    map(fn: (v: any) => any): Expr;
  }
}

function mapDict(this: NativeExpr, mapping: Record<string, string>): NativeExpr {
  // Validate mapping: keys and values must be strings for this implementation.
  if (typeof mapping !== "object" || mapping === null) {
    throw new TypeError("mapDict expects a plain object mapping string -> string");
  }
  // Call into native binding to create a new Expr that performs the mapping.
  // The native binding should convert `mapping` into a native HashMap and build the Expr.
  return native.add_map_dict_expr(this, mapping);
}

function map(this: NativeExpr, fn: (v: any) => any): NativeExpr {
  if (typeof fn !== "function") {
    throw new TypeError("map expects a function");
  }
  // This requires complex native wiring (threadsafe function, serialization).
  // The native binding should create the threadsafe function and return an Expr wrapper.
  return native.add_map_callback_expr(this, fn);
}

(NativeExpr.prototype as any).mapDict = mapDict;
(NativeExpr.prototype as any).map = map;

export {};
