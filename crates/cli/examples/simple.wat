;; Simple WASM Component example
;; This component exports a simple "add" function that adds two numbers

(component
  (type (;0;) (func (param "a" s32) (param "b" s32) (result s32)))
  (export "add" (func 0))
  (func (;0;) (type 0) (param s32 s32) (result s32)
    local.get 0
    local.get 1
    i32.add)
)
