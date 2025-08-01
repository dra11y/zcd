# `zcd`: A smarter cd command that actually works

## The Problem

`zoxide` remembers your directories but **fails at tab completion**. This affects many users:

- **180,000+ [crates.io](https://crates.io/crates/zoxide) downloads**
- **97,000+ [Homebrew](https://formulae.brew.sh/formula/zoxide) installs** in the last year
- **147 packages** across Linux distributions
- **104 open issues** suggests high demand for a fix

## What Actually Happens

You install `zoxide` expecting `z mydir<TAB>` to complete to your frequently used directories. Instead you get:

- **Missing or broken tab completion** - [(#471)](https://github.com/ajeetdsouza/zoxide/issues/471), [(#485)](https://github.com/ajeetdsouza/zoxide/issues/485), [(#766)](https://github.com/ajeetdsouza/zoxide/issues/766)
- **Weird space+tab requirement** - [61👍 (#9)](https://github.com/ajeetdsouza/zoxide/issues/9), [(#325)](https://github.com/ajeetdsouza/zoxide/issues/325), [(#692)](https://github.com/ajeetdsouza/zoxide/issues/692)
- **Broken matching & fzf dependency** - [(#247)](https://github.com/ajeetdsouza/zoxide/issues/247), [(#626)](https://github.com/ajeetdsouza/zoxide/issues/626), [(#727)](https://github.com/ajeetdsouza/zoxide/issues/727)

## Why It's Broken

`zoxide` generates ~150 lines of shell code for each of 9 shells with external fzf dependency and conflicts with existing `cd` setups.

---

## `zcd`: `zoxide` forked and fixed

### Core Idea

`zcd` is a drop-in replacement for `cd` that actually works.

- **`zcd` binary**: Rust executable handling database and matching
- **`z()` function**: Thin shell wrapper (3 lines vs zoxide's ~150)
- **`alias cd=z`**: Seamless replacement

**Git-Style Tab Completion**:
```bash
z mydir<TAB>        # → z mydirectory      (single match)
z my<TAB>           # → Shows: mydirectory, myproject3, my_temp/
z my<TAB><TAB>      # → Cycles through matches
```


### zoxide vs `zcd`

| Feature | zoxide | `zcd` |
|---|---|---|
| **Tab completion** | ❌ Doesn't work [(#471)](https://github.com/ajeetdsouza/zoxide/issues/471) | ✅ Works like `git checkout` |
| **Matching** | ❌ Database only, wrong dirs [(#247)](https://github.com/ajeetdsouza/zoxide/issues/247) | ✅ Database + current directory |
| **Completion behavior** | ❌ `<SPACE><TAB>` weirdness [(#9)](https://github.com/ajeetdsouza/zoxide/issues/9) | ✅ Standard `git`-style menu + cycling |
| **External dependencies** | ❌ fzf + complex shell scripts | ✅ None! Pure Rust |
| **Shell integration** | ❌ 150 lines × 9 shells | ✅ 3-line wrapper |

### The Fix

```bash
# What you expect to work:
z proj<TAB>  # → z project1/

# What zoxide gives you:
z proj<TAB>  # → nothing (broken)
z proj <TAB> # → fzf menu (weird)

# What zcd delivers:
z proj<TAB>     # → z project1/
z proj<TAB><TAB> # → z project2/ (cycles)
```
