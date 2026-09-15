# PROMPT_HISTORY

2026-08-01: Read this project and create comprehensive readme. This is crate for
the new-type pattern, borrowing the concept of `friend classes` from c++.

---

2026-08-09: This project has evolved, rewrite the readme, and do not make it
fluff or an ad. Just say what's important, and more importantly try to
demonstrate with code example rather than text.

---

2026-09-15: Check the readme, do we need to update anything? I don't want the
readme to be like an advertisement. I want practical usage info there. Maybe a
short description why using this crate is a good idea too.

---

Some other crappy prompts. Please excuse typos, unofficial tone, and ...
In no particular order:

- Now write tests for bitflag too, put in separate test file
- I want to assess performance of this project when built with rust nightly, in
  full optimization release mode. Do its promises hold? That the type wrapper
  dissolves at runtime? That is, everything is inlined?
- I want to create a (cargo) unpublished create, let's name it typekin_perf. And
  what you did here, must be put in that crate so it can be run as a test to
  compare and diff the asm and report if basic operations for const types on
  nightly, produce different assembly (they should not). do not test flags yet,
  focus only on integral, and only focus on already constructed values (that is,
  do not test friendship or of (...) methods), only tests math operations and
  bit operations for now.
- Yep, then do that. Create these crates: typekin_perf_types -> which only
  contains type definitions and usage of the integral proc-macro. Then create
  typekin_perf_must_fail, that builds without optimization. We expect the
  assembly to differ. Then create the crate typekin_perf_ok which is basically
  the current typekin_perf you created.
