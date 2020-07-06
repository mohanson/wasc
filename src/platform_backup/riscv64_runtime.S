.extern wavm_intrinsic_memory_grow

.global "memory.grow"
.set "memory.grow", .
  j wavm_intrinsic_memory_grow

.global tableReferenceBias
.set tableReferenceBias, -32
