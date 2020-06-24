(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func))
  (import "wasi_unstable" "args_get" (func (;0;) (type 0)))
  (import "wasi_unstable" "args_sizes_get" (func (;1;) (type 0)))
  (import "wasi_unstable" "fd_write" (func (;2;) (type 1)))
  (import "wasi_unstable" "proc_exit" (func (;3;) (type 2)))
  (func (;4;) (type 3) (param i32) (result i32)
    (local i32)
    local.get 0
    i32.const 15
    i32.add
    i32.const -16
    i32.and
    local.set 0
    global.get 5
    local.set 1
    local.get 1
    local.get 0
    i32.add
    global.set 5
    local.get 1)
  (func (;5;) (type 3) (param i32) (result i32)
    (local i32)
    local.get 0
    local.set 1
    loop  ;; label = @1
      block  ;; label = @2
        i32.const 0
        local.get 1
        i32.load8_u
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 1
    local.get 0
    i32.sub
    return)
  (func (;6;) (type 4)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      global.get 2
      global.get 3
      call 1
      local.set 0
      local.get 0
      i32.const 0
      i32.ne
      br_if 0 (;@1;)
      global.get 2
      i32.load
      local.set 1
      global.get 3
      i32.load
      local.set 2
      local.get 1
      i32.const 4
      i32.mul
      call 4
      local.set 3
      local.get 2
      call 4
      local.set 4
      local.get 3
      local.get 4
      call 0
      local.set 0
      local.get 0
      i32.const 0
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 1
      i32.sub
      i32.const 2
      i32.mul
      local.set 6
      local.get 6
      i32.const 8
      i32.mul
      call 4
      local.set 5
      local.get 5
      local.set 8
      i32.const 1
      local.set 7
      block  ;; label = @2
        loop  ;; label = @3
          local.get 7
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          local.get 3
          local.get 7
          i32.const 4
          i32.mul
          i32.add
          i32.load
          local.set 9
          local.get 9
          call 5
          local.set 10
          local.get 8
          local.get 9
          i32.store
          local.get 8
          local.get 10
          i32.store offset=4
          local.get 8
          global.get 1
          global.get 0
          local.get 1
          local.get 7
          i32.const 1
          i32.add
          i32.eq
          select
          i32.store offset=8
          local.get 8
          i32.const 1
          i32.store offset=12
          local.get 7
          i32.const 1
          i32.add
          local.set 7
          local.get 8
          i32.const 16
          i32.add
          local.set 8
          br 0 (;@3;)
        end
      end
      i32.const 1
      local.get 5
      local.get 6
      global.get 4
      call 2
      local.set 0
    end
    local.get 0
    call 3)
  (memory (;0;) 1)
  (global (;0;) i32 (i32.const 0))
  (global (;1;) i32 (i32.const 1))
  (global (;2;) i32 (i32.const 4))
  (global (;3;) i32 (i32.const 8))
  (global (;4;) i32 (i32.const 12))
  (global (;5;) (mut i32) (i32.const 128))
  (export "memory" (memory 0))
  (export "_start" (func 6))
  (data (;0;) (i32.const 0) " ")
  (data (;1;) (i32.const 1) "\0a"))
