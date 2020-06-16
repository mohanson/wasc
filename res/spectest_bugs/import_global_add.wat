(module
 (import "spectest" "global_i32" (global $global_i32 i32))
 (global $zen i32 (i32.const 10))
 (export "main" (func $main))
 (func $main (; 0 ;) (result i32)
  (i32.add
   (get_global $global_i32)
   (get_global $zen)
  )
 )
)
