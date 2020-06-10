.extern wavm_intrinsic_memory_grow

.global "memory.grow"
.set "memory.grow", .
  jmp wavm_intrinsic_memory_grow
