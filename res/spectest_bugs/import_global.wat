(module
 (import "spectest" "global_i32" (global $global_i32 i32))
 (export "main" (func $main))
 (func $main (; 0 ;) (result i32)
   (get_global $global_i32)
 )
)
