(module
  (func $add (result i32)
    i32.const 42
    i32.const 58
    i32.add)
  (export "add" (func $add))
)