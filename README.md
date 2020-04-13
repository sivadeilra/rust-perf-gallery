# Rust Performance Gallery

This crate is a gallery of Rust code idioms, meant to illustrate how they map to assembly code.
This is intended to help people understand how to write Rust code that generates efficient
assembly code.

Do not prematurely optimize! Only optimize code after you have evidence that the "optimizations"
are actually worth it. Do not distort your source code for no benefit. It is really easy to make
the mistake of thinking that a performance "optimization" is worth it, when very often it is not.

However, sometimes performance optimization is necessary. There are several different ways we can
optimize Rust code:

* Learning how to use Rust better, and writing _better_ Rust code (more idiomatic code) which
  translates to _better_ machine code. For example, code which uses `Iterator`-based types very
  often performs _better_ than the equivalent code which manually implements the same thing, 
  using array indices. Learning how to use `Iterator`-based combinators will almost always improve
  your Rust code, while _also_ improving the efficiency of the generated machine code.

* Learning how the Rust optimizer analyzes your code. (Note: In this document, I will use "Rust" to
  mean both the Rust front-end and the LLVM back-end. They work as a system to compile your code
  to machine code.)
  
  You do not need to be a compiler expert, but a basic mental model of how optimization works can
  often enable you to write efficient Rust code. For example, knowing when Rust can safely omit
  bounds checks can allow you to write smaller, faster code.

# Potential performance improvements

## Finding a more efficient idiom for panicking

Each panic path generates X? bytes of instructions. How can we reduce this? Ideally, we should have
a compilation mode which indicates "I don't need information about each specific panic site.
I need the most efficient code possible."

Possible idioms:

* Generate a single panic branch in each function. All branches within that function jump to it.
  + Panicks are mapped to each function, but without knowing which branch caused the
    panic.

* Generate a single panic branch per group of functions, limited only by the limits of relative
  jump instruction encodings.
  + Advantage: Small code.
  + Disadvantage: Call stacks will be confusing. They will not show the faulting function.

* Use `CMOVcc` to read from a memory address that we know is unmapped. Example:

  ```
  xor rsi, rsi                   ; clear rsi to load a NULL pointer  <-- shareable
  cmp rax, 16
  cmova rbx, qword ptr[rsi]      ; deref null if the bounds check fails
  ```

  + Advantage: Very small idiom
  + Advantage: No separate BB needed
  + Advantage: Faulting address is within the BB that caused the panic. Debugging will be easy.
  + Disadvantage: Burns one virtual register. However, since it's just an xor, it allocates a
    register name, but not an underlying physical register.
  + Disadvantage: In a debugger, this looks like a NULL dereference. This could be mitigated by
    using a vectored exception handler, which inspects the faulting code. If the faulting code is
    a CMOVcc instruction, then we treat this exception as a Rust panic.
  + Potential disadvantage: There may be strange performance effects around `CMOV` instructions.
    See Linus Torvalds' post on `CMOV`: https://yarchive.net/comp/linux/cmov.html
    This may be outdated by modern processors, but it is worth considering. `CMOV` is not widely
    used, and so may not be widely optimized.

  + NOPE: It looks like `CMOV` _always_ loads its source into a "temporary register", so it would
    always generate a fault.

* Keep using `Jxx` to a BB, but generate a single panic BB per function, and use one register to
  indicate which BB it came from:

  ```
  mov rip, r15                  ; save instruction of potentially-panicking code
  cmp rax, 16                   ; test condition
  ja panic_handler

  ...
  panic_handler:
    jmp global_panic_handler    ; global_panic_handler knows that r15 holds faulting ip
  ```

  + Advantage: Very small idiom. Shared panic handler BB is also very small, when needed.
  + Advantage: Takes full advantage of branch prediction, and so avoids any potential worries from
    `CMOV`.
  + Advantage: Code makes sense to a reader / debugger. No weird "fake NULL deref".
  + It might be possible to represent this in ordinary LLVM IR, and so this could be done in rustc,
    without changes to LLVM.
  + Disadvantage: Requires allocating a register, and saving RIP may have a cost. This may be
    an unacceptable performance cost.

* Keep using `Jxx` but use it for the non-panicking side of the jump, and jump over the
  panic-handler code:

  ```
      cmp rax, 16
      jna everything_is_ok
  panic_handler:
      int 0x29
  everything_is_ok:
      ...                           ; more good code

  ```

  + Advantage: small, etc.
  + Disadvantage: Contradicts Intel's guidance about static branch prediction. When using this
    idiom, the CPU will predict that the panic branch is always taken.
