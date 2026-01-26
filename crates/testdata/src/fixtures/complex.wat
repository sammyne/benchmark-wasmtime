(module
  ;; Fibonacci function: computes fib(10) = 55
  (func $fibonacci (result i32)
    (local $n i32)
    (local $a i32)
    (local $b i32)
    (local $temp i32)
    
    ;; Initialize n = 10
    i32.const 10
    local.set $n
    
    ;; Initialize a = 0, b = 1
    i32.const 0
    local.set $a
    i32.const 1
    local.set $b
    
    ;; Loop for n iterations
    (block $break
      (loop $continue
        ;; if n == 0, break
        local.get $n
        i32.eqz
        br_if $break
        
        ;; temp = a + b
        local.get $a
        local.get $b
        i32.add
        local.set $temp
        
        ;; a = b
        local.get $b
        local.set $a
        
        ;; b = temp
        local.get $temp
        local.set $b
        
        ;; n = n - 1
        local.get $n
        i32.const 1
        i32.sub
        local.set $n
        
        br $continue
      )
    )
    
    ;; Return b (fibonacci result)
    local.get $b)
  (export "fibonacci" (func $fibonacci))
)