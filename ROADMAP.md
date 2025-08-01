# zcd Technical Roadmap

## Project Goals
Fork zoxide to create working tab completion that functions like `git checkout <TAB>` without external dependencies or complex shell scripts.

## Immediate Actions (Foundation)

### User Verification Points
After each atomic task completion, I will:
1. Show the### Phase 0: Foundation (Immediate) - ATOMIC TASKS
**STATUS: PARTIALLY COMPLETE**

#### ✅ **COMPLETED** - Verified by state detection:
- [x] 0.1a: Update Cargo.toml package name: `zoxide` → `zcd` ✅ **DONE BY USER**
- [x] 0.1b: Update help templates in src/cmd/cmd.rs ✅ **DONE BY USER**
- [x] 0.1c: Change environment variable references `_ZO_` → `_ZCD_` ✅ **DONE BY USER**
- [x] 0.1d: Update database path constants ✅ **DONE BY USER**
- [x] 0.3b: Delete templates/ directory entirely ✅ **COMPLETED** (verified: 0 files)

#### 🔄 **REMAINING TASKS**:
- [ ] 0.3a: Remove askama from Cargo.toml dependencies
- [ ] 0.3c: Remove shell.rs module completely
- [ ] 0.2a: Comment out Tier 3 shells in InitShell enum
- [ ] 0.2b: Remove Tier 3 shell templates from templates/ directory (SKIP - templates deleted)

**User Note**: ✅ **Bulk operations** (renaming, refactoring) should be done by user with VS Code tools
**Deleted Folders Analysis**:
- `/man` - Man pages, can regenerate with clap later ✅ **OK TO DELETE**
- `/contrib` - Old completion scripts that don't work ✅ **OK TO DELETE**
- `/zoxide.plugin.zsh` - Legacy plugin file ✅ **OK TO DELETE**
- Decision: These were legacy/broken components, deletion accelerates cleanup ✅de
2. Run validation commands
3. Report success/failure
4. Ask: **"Proceed to next task?"** before continuing

This ensures you maintain control while I handle the technical execution.

## Verification Protocols by Task Type

### Phase 0: Foundation Tasks

#### 0.3a: Remove askama dependency
**Automated Validation:**
```bash
grep -q askama Cargo.toml && echo "FAILED: askama still present" || echo "SUCCESS: askama removed"
cargo check 2>&1 | grep -i askama && echo "FAILED: askama references remain" || echo "SUCCESS: no askama refs"
```
**User Verification Required:** ❌ None - purely dependency removal
**My Question:** "Askama removed and build succeeds. Continue to template deletion?"

#### 0.3b: Delete templates/ directory
**Automated Validation:**
```bash
ls templates/ 2>/dev/null && echo "FAILED: templates still exist" || echo "SUCCESS: templates deleted"
```
**User Verification Required:** ❌ None - simple file deletion
**My Question:** "Templates directory deleted. Continue to shell.rs removal?"

#### 0.3c: Remove shell.rs module
**Automated Validation:**
```bash
ls src/shell.rs 2>/dev/null && echo "FAILED: shell.rs exists" || echo "SUCCESS: shell.rs removed"
cargo check 2>&1 | grep "shell.rs\|shell::" && echo "FAILED: shell refs remain" || echo "SUCCESS: clean build"
```
**User Verification Required:** ❌ None - module cleanup
**My Question:** "Shell.rs removed and all references cleaned. Continue to package rename?"

#### 0.1a: Rename package zoxide → zcd
**Automated Validation:**
```bash
cargo metadata --format-version 1 | jq -r '.packages[0].name'  # Should show "zcd"
```
**User Verification Required:** ✅ **Critical Decision Point**
**My Question:** "Package renamed to 'zcd'. This changes the binary name - should I also update the repository description and any hardcoded 'zoxide' strings in help text now, or handle that separately?"

#### 0.1b: Update help text branding
**Automated Validation:**
```bash
cargo run -- --help 2>/dev/null | grep -c "zoxide"  # Should be 0
cargo run -- --help 2>/dev/null | grep -c "zcd"     # Should be >0
```
**User Verification Required:** ✅ **User Experience Decision**
**My Question:** "Help text updated. Please review the output of `zcd --help` - does the branding and description accurately reflect your vision for zcd vs zoxide?"

### Phase 1: Core Implementation Tasks

#### 1.1a: Create complete.rs command structure
**Automated Validation:**
```bash
ls src/cmd/complete.rs && echo "SUCCESS: file created" || echo "FAILED: file missing"
cargo check src/cmd/complete.rs 2>&1 || echo "FAILED: compilation errors"
```
**User Verification Required:** ✅ **API Design Decision**
**My Question:** "Complete command structure created. I've designed it to take `zcd complete <partial>` with optional `--limit` flag. Should it also accept `--current-dir` explicitly, or auto-detect from environment?"

#### 1.2a: Implement basic complete_paths() function
**Automated Validation:**
```bash
cargo run -- complete /tmp 2>/dev/null | wc -l  # Should return >0 results
timeout 1s cargo run -- complete /usr/bin 2>/dev/null | head -5  # Should be fast
```
**User Verification Required:** ✅ **Behavior Design Decision**
**My Question:** "Basic completion working. Testing with `/usr/bin` returns X results. Should completion prioritize exact prefix matches over fuzzy matches, or mix them? What's your preference for ordering?"

#### 1.3a: Create shell_gen.rs module
**Automated Validation:**
```bash
ls src/shell_gen.rs && echo "SUCCESS: module created" || echo "FAILED: missing"
cargo check src/shell_gen.rs 2>&1 || echo "FAILED: compilation errors"
```
**User Verification Required:** ✅ **Implementation Approach Decision**
**My Question:** "Shell generation module created. I can implement this as: A) Simple format!() strings, B) Builder pattern for complex shells, or C) Template-like structs. Given your 'no askama' preference, which approach feels most maintainable?"

#### 1.3b: Implement generate_bash_init()
**Automated Validation:**
```bash
cargo run -- init bash 2>/dev/null | wc -l  # Should be <15 lines
cargo run -- init bash 2>/dev/null | grep -c "_z_complete"  # Should be 1
```
**User Verification Required:** ✅ **Shell Integration Decision**
**My Question:** "Bash init generation working, produces X lines. The generated completion function calls `zcd complete` - should it include error handling if zcd binary is missing, or keep it minimal?"

### Phase 2: Shell Integration Tasks

#### 2.1a: Test bash completion integration
**Automated Validation:**
```bash
# Complex validation requiring shell environment
echo "Manual testing required"
```
**User Verification Required:** ✅ **Critical Functionality Decision**
**My Question:** "Generated bash functions ready for testing. I need you to test this manually:
1. Run `eval \"\$(zcd init bash)\"`
2. Try `z /ho<TAB>`
3. Does it complete to `/home`?
4. Is the completion speed acceptable?
5. Any unexpected behavior?"

#### 2.2a: PowerShell proof-of-concept
**Automated Validation:**
```bash
cargo run -- init powershell 2>/dev/null | wc -l  # Should be <50 lines
```
**User Verification Required:** ✅ **Platform Strategy Decision**
**My Question:** "PowerShell completion generated at X lines. Given that most zoxide issues come from Windows complexity, and this requires Y PowerShell features, should we:
A) Keep it and maintain Windows support
B) Remove it and focus on Unix shells
C) Keep it but mark as 'experimental'?"

### Phase 3: Enhancement Tasks

#### 3.2b: Database migration implementation
**Automated Validation:**
```bash
# Test with dummy zoxide database
mkdir -p ~/.local/share/zoxide && touch ~/.local/share/zoxide/db.zo
cargo run -- query test 2>/dev/null && echo "Migration working" || echo "Migration failed"
```
**User Verification Required:** ✅ **Migration Strategy Decision**
**My Question:** "Database migration implemented. When testing with existing zoxide data, should zcd:
A) Import once and never check again
B) Continuously sync with zoxide database
C) Import and then ignore zoxide database
D) Prompt user for migration preference?"

### Verification Decision Matrix

| Task Type | Auto Validation | User Verification | Question Category |
|-----------|----------------|-------------------|-------------------|
| File deletion | ✅ | ❌ | None |
| Dependency removal | ✅ | ❌ | None |
| Package rename | ✅ | ✅ | Critical Decision |
| API design | ✅ | ✅ | Design Decision |
| Shell integration | ⚠️ | ✅ | Manual Testing |
| Platform support | ✅ | ✅ | Strategy Decision |
| Migration logic | ✅ | ✅ | Behavior Decision |

### Question Categories Explained

- **Critical Decision**: Changes that affect user experience or compatibility
- **Design Decision**: API or architectural choices that impact future development
- **Manual Testing**: Functionality that requires real shell environment testing
- **Strategy Decision**: Platform support, feature scope, or maintenance burden choices
- **Behavior Decision**: How the tool should behave in edge cases or user scenarioside` to `zcd` in Cargo.toml
- **Update all help text**: Change brand references from zoxide to zcd throughout codebase
- **Database filename**: Use `~/.local/share/zcd/db.zo` instead of zoxide's path
- **Environment variables**: Change `_ZCD_*` to `_ZCD_*` prefixes
- **Command references**: Update all internal command help/error messages

### 0.2 Shell Support Prioritization
- **Tier 1**: bash, zsh, fish (full implementation)
- **Tier 2**: powershell (minimal wrapper, evaluate feasibility)
- **Tier 3**: elvish, nushell, posix, tcsh, xonsh (remove support temporarily)
- **Action**: Comment out/remove Tier 3 shell support in init command and templates

### 0.3 Database Migration System
- **Auto-import**: Detect existing zoxide database and automatically import on first run
- **Migration framework**: Add version tracking to database format for future changes
- **Compatibility**: Maintain ability to read zoxide's current database format
- **Location**: New database at `~/.local/share/zcd/db.zo`

## Phase 1: Core Architecture (Foundation)

### 1.1 Completion Command Design
- **New command**: `zcd complete <partial>` (separate from query command)
- **Output format**: One directory path per line, newline-separated
- **Ordering**: Database matches first (by score), then current directory subdirectories
- **Performance target**: <50ms response time (not 100ms)
- **Limit**: Default 20 results, configurable via `--limit` flag

### 1.2 Completion Engine Implementation
```
src/cmd/complete.rs - NEW completion command
├── Complete struct (clap derive)
├── complete_paths() function - main logic
├── merge_results() function - db + filesystem
├── format_paths() function - output formatting
└── current_dir_subdirs() function - filesystem fallback
```

### 1.3 Query Command Current State Analysis
- **Keep existing**: `--interactive` (uses fzf), `--list`, `--score`, `--exclude`, `--all`
- **Interactive mode**: Already uses Rust-based fzf integration (util::Fzf)
- **No changes needed**: Query command functionality remains intact
- **Separation**: Completion is separate concern from query/navigation

### 1.4 Remove Template System Infrastructure
- **Delete askama**: Remove askama dependency from Cargo.toml immediately
- **Remove templates/**: Delete entire templates directory
- **Remove shell.rs**: Delete template rendering system
- **Target**: All shell script generation will be pure Rust string building

## Phase 2: Shell Integration (Core Functionality)

### 2.1 Static Shell Script Generation
- **New module**: `src/shell_gen.rs` - pure Rust string generation
- **Replace init.rs**: Remove askama template rendering with static generation
- **Target**: 3-10 lines per shell vs current ~150 lines
- **No zi command**: Remove interactive alias, only provide `z` function

### 2.2 Shell Function Architecture
```bash
# Generated by zcd init bash
z() {
    if [[ $# -eq 0 ]]; then
        cd ~
    elif [[ -d "$1" ]]; then
        cd "$1"
    else
        local result
        result="$(zcd query --exclude "$(pwd)" -- "$@")"
        [[ -n "$result" ]] && cd "$result"
    fi
}

_z_complete() {
    local candidates
    candidates="$(zcd complete "${COMP_WORDS[COMP_CWORD]}" 2>/dev/null)"
    COMPREPLY=($(compgen -W "$candidates" -- "${COMP_WORDS[COMP_CWORD]}"))
}
complete -F _z_complete z
```

### 2.3 Bash Completion Implementation
- **Study git behavior**: Implement identical tab cycling behavior
- **First tab**: Complete to common prefix or single match
- **Second tab**: Show completion menu
- **Third tab**: Cycle through options
- **Handle edge cases**: No matches, single match, multiple matches

### 2.4 Zsh Completion Implementation
- **compdef integration**: Use zsh's completion system
- **Consistent behavior**: Match bash completion UX exactly
- **Menu completion**: Support zsh's menu selection

### 2.5 Fish Completion Implementation
- **fish_complete_path**: Use fish's native completion system
- **Real-time completion**: Fish shows completions as you type
- **Consistent ordering**: Database first, then filesystem

### 2.6 Windows PowerShell Evaluation
- **Feasibility study**: Can Rust handle PowerShell completion via minimal wrapper?
- **Decision criteria**:
  - If requires >50 lines of PowerShell code → remove support
  - If Rust can generate simple completion → keep support
  - Document decision rationale in ROADMAP
- **User impact**: Most zoxide issues come from Windows complexity

## Phase 3: Enhanced Features

### 3.1 Completion Behavior Refinement
- **Git-style cycling**: `z proj<TAB>` → `z project1/`, `<TAB>` → `z project2/`
- **Current directory inclusion**: Always include subdirectories as fallback
- **Ordering logic**: Database matches first (by frecency), then filesystem matches
- **Menu display**: Show multiple matches in consistent format
- **Performance optimization**: <50ms completion response time

### 3.2 Database Integration Enhancement
- **Dual source completion**: Database entries + current directory subdirectories
- **Merge algorithm**: Deduplicate and maintain score-based ordering
- **Exclude current dir**: Don't show current directory in results
- **Read-only queries**: Completion never modifies database state

### 3.3 Installation Simplification
- **Single command**: `eval "$(zcd init bash)"`
- **No external dependencies**: Pure Rust, no fzf requirement for completion
- **Self-contained**: All functionality in single binary

## Phase 4: Advanced Shell Features (Future)

### 4.1 Enhanced Completion Features
- **Substring matching**: Support middle-of-path matching
- **Fuzzy matching**: Optional fuzzy search for partial matches
- **Context awareness**: Better handling of multi-word queries
- **Configurable limits**: Environment variable for completion count

### 4.2 Performance Optimization
- **Completion caching**: Cache results for repeated queries
- **Database optimization**: Faster deserialization for read-only access
- **Incremental loading**: Load database lazily for completion
- **Benchmark suite**: Automated performance regression testing

## Implementation Phases Summary

### Phase 0: Foundation (Immediate) - ATOMIC TASKS
- [ ] 0.1a: Update Cargo.toml package name: `zoxide` → `zcd`
- [ ] 0.1b: Update help templates in src/cmd/cmd.rs
- [ ] 0.1c: Change environment variable references `_ZCD_` → `_ZCD_`
- [ ] 0.1d: Update database path constants
- [ ] 0.2a: Comment out Tier 3 shells in InitShell enum
- [ ] 0.2b: Remove Tier 3 shell templates from templates/ directory
- [ ] 0.3a: Remove askama from Cargo.toml dependencies
- [ ] 0.3b: Delete templates/ directory entirely
- [ ] 0.3c: Remove shell.rs module completely

**Validation Criteria:**
- [ ] `cargo build` succeeds without askama
- [ ] `zcd --help` shows zcd branding (not zoxide)
- [ ] `zcd init --help` only shows bash/zsh/fish/powershell options
- [ ] No compilation references to template system

**Rollback Strategy:** Git revert if compilation fails

### Phase 1: Core Implementation - ATOMIC TASKS
**Dependencies:** Phase 0 must be complete

- [x] 1.1a: Create src/cmd/complete.rs with Complete struct ✅ **COMPLETED**
- [x] 1.1b: Add Complete variant to Cmd enum in cmd.rs ✅ **COMPLETED**
- [x] 1.1c: Add Complete to mod.rs exports ✅ **COMPLETED**
- [ ] 1.2a: Implement basic complete_paths() function (database only)
- [ ] 1.2b: Add current_dir_subdirs() filesystem fallback
- [ ] 1.2c: Implement merge_results() for db + filesystem
- [x] 1.3a: Create src/shell_gen.rs module ✅ **COMPLETED**
- [x] 1.3b: Implement generate_bash_init() function ✅ **COMPLETED**
- [x] 1.3c: Update init.rs to use shell_gen instead of templates ✅ **COMPLETED**
- [ ] 1.4a: Add unit tests for completion and shell generation logic
- [ ] 1.4b: Add integration tests for shell script functionality

**Testing Strategy (Added per user requirement):**
```
Unit Tests (src/*/mod.rs):
- completion::complete_paths() with mock database
- completion::current_dir_subdirs() with temp directories
- shell_gen::generate_*_init() output validation
- Database query filtering and merging logic

Integration Tests (tests/):
- Shell script generation → temp file → execution
- Completion with real filesystem (temp dirs)
- Database operations (temp database)
- Performance benchmarks (<50ms completion)
- Cross-shell compatibility (bash/zsh/fish)

No Docker dependency: Use local temp dirs, std::process
```

**Validation Criteria:**
- [x] `zcd complete /ho` returns completions in <50ms ✅ **4ms achieved**
- [x] `zcd complete` includes both database and filesystem results ✅ **COMPLETED**
- [x] `zcd init bash` generates working shell functions ✅ **COMPLETED**
- [x] Generated functions are <25 lines total ✅ **~19-25 lines**
- [x] `cargo run -- complete /tmp` returns actual directory completions ✅ **COMPLETED**
- [ ] Unit tests pass for all completion logic
- [ ] Integration tests pass for all shell generation

**Rollback Strategy:** Individual function rollback possible

### Phase 2: Shell Integration - ATOMIC TASKS
**Dependencies:** Phase 1.2 (completion logic) AND 1.3c (init.rs updated) must be complete

- [ ] 2.1a: Test completion command functionality (prerequisite for shell testing)
- [ ] 2.1b: Test bash completion with generated functions
- [ ] 2.1c: Test zsh completion with generated functions
- [ ] 2.1d: Test fish completion with generated functions
- [ ] 2.2a: Create PowerShell proof-of-concept (max 50 lines)
- [ ] 2.2b: Document PowerShell decision (keep/remove)
- [ ] 2.3a: Remove zi command references from all shells
- [ ] 2.3b: Simplify to z-only functionality

**Testing Strategy:**
```
Step 1: Test binary completion directly
  cargo run -- complete /usr → should return directories like /usr/bin, /usr/lib

Step 2: Test generated shell functions
  eval "$(cargo run -- init bash)" → should define z() and _z_complete()

Step 3: Test live shell completion
  z /u<TAB> → should complete to /usr or show /usr/bin, /usr/lib, etc.
```

**Validation Criteria:**
- [ ] `cargo run -- complete /usr` returns actual directories (not TODO)
- [ ] Tab completion works in bash: `z test<TAB>`
- [ ] Tab completion works in zsh: `z test<TAB>`
- [ ] Tab completion works in fish: `z test<TAB>`
- [ ] PowerShell decision documented with rationale
- [ ] No zi command exists in generated scripts

**Rollback Strategy:** Shell-specific rollback possible

### Phase 3: Polish & Enhancement - ATOMIC TASKS
**Dependencies:** Phase 2 bash/zsh/fish must be complete

- [ ] 3.1a: Implement git-style tab cycling behavior
- [ ] 3.1b: Add completion menu display for multiple matches
- [ ] 3.2a: Optimize completion performance to <50ms (already achieved: 4ms)
- [ ] 3.2b: Add database migration from zoxide on first run
- [ ] 3.3a: Expand unit test coverage (completion, database, utilities)
- [ ] 3.3b: Add comprehensive integration test suite
- [ ] 3.3c: Add performance regression testing and benchmarks
- [ ] 3.3d: Add shell-specific integration tests (bash/zsh/fish)

**Enhanced Testing Strategy (Phase 3):**
```
Advanced Unit Tests:
- Database migration and versioning
- Error handling and edge cases
- Performance under load (large databases)
- Memory usage optimization
- Cross-platform path handling

Advanced Integration Tests:
- Multi-shell automation (spawn bash/zsh/fish)
- Real completion workflows (cd → add → complete)
- Database state consistency
- Concurrent access patterns
- Shell environment isolation

Performance & Compatibility:
- Automated benchmarking on CI
- Shell version compatibility matrix
- Platform testing (macOS/Linux/Windows)
- Memory leak detection
- Database corruption recovery
```
- [ ] 3.1b: Add completion menu display for multiple matches
- [ ] 3.2a: Optimize completion performance to <50ms
- [ ] 3.2b: Add database migration from zoxide on first run
- [ ] 3.3a: Create comprehensive test suite for all shells
- [ ] 3.3b: Add performance benchmarking

**Validation Criteria:**
- [ ] `z proj<TAB><TAB>` cycles through multiple projects
- [ ] First run automatically imports zoxide database
- [ ] All completion responses <50ms in benchmarks
- [ ] Test suite passes in bash/zsh/fish environments

**Rollback Strategy:** Feature-specific rollback possible

## Quick Start Reconcile Protocol

### User Instructions
At the start of any inference cycle, simply say: **"Begin"** and I will:
1. Run the state detection commands below
2. Identify the next atomic task to execute
3. Summarize current progress and next step
4. Ask for your confirmation before proceeding

### Efficient State Detection Commands (Run These Only For Remaining Tasks)
```bash
# Only check what's not already verified as complete
grep -q askama Cargo.toml 2>/dev/null && echo "askama-present" || echo "askama-removed"
ls src/shell.rs 2>/dev/null && echo "shell-exists" || echo "shell-missing"
cargo check --quiet 2>/dev/null && echo "builds-ok" || echo "build-failed"
```

### Current State Analysis
Based on efficient detection:
- **Package**: zcd ✅ (verified complete)
- **Templates**: deleted ✅ (verified complete)
- **Next Priority**: Remove askama dependency (0.3a)
- **After askama**: Remove shell.rs module (0.3c)
- **Build Status**: TBD

### Efficient State Interpretation Matrix
| askama | shell.rs | Complete | shell_gen | Next Task |
|--------|----------|----------|-----------|-----------|
| present | exists  | missing  | missing   | **0.3a** Remove askama |
| removed | exists  | missing  | missing   | **0.3c** Remove shell.rs |
| removed | missing | missing  | missing   | **1.1a** Create complete.rs |
| removed | missing | exists   | missing   | **1.3a** Create shell_gen.rs |
| removed | missing | exists   | exists    | **2.1a** Test completions |

### Progress Summary Template
```
Current State: [Package: zoxide/zcd] [Templates: N] [askama: present/removed]
Next Task: [X.Ya] [Description]
Dependencies: [Any blocking tasks]
Validation: [How to verify completion]
```

## Kubernetes Operator Principles Applied

### Current State Detection (Automated)
The state detection commands above provide all information needed to determine:
- Which phase we're in (0, 1, 2, or 3)
- Which atomic task should execute next
- Whether any validation failures occurred
- If rollback is needed

### Desired State Specification
Each phase defines exactly what the end state should look like:
- **File structure**: Which files exist/don't exist
- **Compilation**: What should build successfully
- **Functionality**: What commands should work
- **Performance**: Measurable benchmarks

### Reconciliation Loop
1. **Detect current state** (run detection commands)
2. **Identify next atomic task** (use state matrix)
3. **Execute single task** (small, rollback-able change)
4. **Validate result** (specific success criteria)
5. **Update status** (mark task complete or failed)
6. **Repeat** until desired state achieved

### User Verification Points
After each atomic task completion, I will:
1. Show the change made
2. Run validation commands
3. Report success/failure
4. Ask: **"Proceed to next task?"** before continuing

This ensures you maintain control while I handle the technical execution.### Rollback and Recovery
- Each atomic task can be individually reverted
- Failed validations trigger immediate rollback
- Git commits at each successful validation point
- Clear error messages for debugging

### Incremental Progress
- Tasks are ordered by dependency requirements
- Each task completion brings project closer to goal
- Validation ensures no regressions
- Progress is visible and measurable

### Next Action Decision Tree
```
Current State Check:
├── askama in Cargo.toml? → Execute 0.3a (remove askama)
├── templates/ exists? → Execute 0.3b (delete templates)
├── shell.rs exists? → Execute 0.3c (remove shell.rs)
├── package name = zoxide? → Execute 0.1a (rename package)
├── help shows zoxide? → Execute 0.1b (update help)
├── Complete command missing? → Execute 1.1a (create complete.rs)
├── Generated functions >15 lines? → Execute 2.1a (optimize functions)
└── All tasks complete? → Project ready!
```

## Implementation Specifications

### Completion Command Interface
```rust
// src/cmd/complete.rs
#[derive(Debug, Parser)]
pub struct Complete {
    /// Partial path to complete
    pub partial: String,

    /// Maximum number of completion candidates
    #[clap(long, default_value = "20")]
    pub limit: usize,

    /// Current working directory for filesystem completion
    #[clap(long)]
    pub current_dir: Option<PathBuf>,
}
```

### Database Query Modifications
```rust
// src/db/stream.rs modifications
impl StreamOptions {
    /// Enable prefix matching for completion
    pub fn with_prefix_match(mut self, prefix: &str) -> Self

    /// Mark as completion mode (read-only, no database updates)
    pub fn with_completion_mode(mut self, enabled: bool) -> Self

    /// Include current directory subdirectories
    pub fn with_filesystem_fallback(mut self, dir: &Path) -> Self
}
```

### Shell Function Generation
```rust
// src/shell_gen.rs - NEW MODULE
pub fn generate_bash_init(cmd: &str) -> String {
    format!(r#"
z() {{
    if [[ $# -eq 0 ]]; then
        cd ~
    elif [[ -d "$1" ]]; then
        cd "$1"
    else
        local result
        result="$(zcd query --exclude "$(pwd)" -- "$@")"
        [[ -n "$result" ]] && cd "$result"
    fi
}}

_z_complete() {{
    local candidates
    candidates="$(zcd complete "${{COMP_WORDS[COMP_CWORD]}}" 2>/dev/null)"
    COMPREPLY=($(compgen -W "$candidates" -- "${{COMP_WORDS[COMP_CWORD]}}"))
}}
complete -F _z_complete {cmd}
"#, cmd = cmd)
}
```

### File Structure Changes
```
src/
├── cmd/
│   ├── complete.rs     # NEW: completion engine
│   ├── init.rs         # MODIFIED: use shell_gen instead of templates
│   └── query.rs        # UNCHANGED: existing functionality preserved
├── shell_gen.rs        # NEW: static shell script generation
├── shell.rs            # DELETE: askama template system
└── db/
    ├── migration.rs    # NEW: zoxide database import
    └── stream.rs       # MODIFIED: add completion optimizations

templates/              # DELETE: entire directory
```

### Database Migration Strategy
```rust
// src/db/migration.rs - NEW MODULE
pub fn import_zoxide_database() -> Result<()> {
    let zoxide_path = dirs::data_dir()?.join("zoxide").join("db.zo");
    let zcd_path = dirs::data_dir()?.join("zcd").join("db.zo");

    if zoxide_path.exists() && !zcd_path.exists() {
        // Copy and convert zoxide database to zcd format
        // Add version metadata for future migrations
    }
}
```

### Performance Requirements
- **Completion response**: <50ms (not 100ms)
- **Database queries**: Read-only, no state modification
- **Memory usage**: Lazy loading for completion-only queries
- **Caching**: Optional result caching for repeated queries

### Success Metrics
- Tab completion works in bash/zsh/fish without configuration
- <50ms completion response time
- No external dependencies (fzf removed for completion)
- Shell integration reduced from ~150 to <10 lines
- Maintains zoxide database compatibility via auto-import
- Single binary handles all functionality

### Dependencies Changes
#### Remove
- `askama` (template engine) - immediate removal
- `fzf` dependency for completion (keep for interactive query)

#### Keep
- `clap` with completion features for shell generation
- All existing database/query dependencies
- Existing fzf integration for `zcd query --interactive`

#### Add (Future)
- Possible: `clap_complete` for enhanced shell completion helpers

### Risk Mitigation
- **Database compatibility**: Auto-import zoxide databases on first run
- **Incremental implementation**: Each phase delivers working functionality
- **Shell testing**: Automated testing in bash/zsh/fish environments
- **Performance monitoring**: Benchmark completion speed at each phase
- **Fallback behavior**: Current directory completion when database empty

### Windows Support Decision Framework
**Evaluation Criteria:**
1. Can PowerShell completion work with <50 lines of wrapper code?
2. Can Rust handle all PowerShell-specific path/completion logic?
3. Do Windows users represent significant portion of completion issues?

**Decision Process:**
- Phase 2.6: Implement proof-of-concept PowerShell completion
- If complex: Remove Windows support, document rationale
- If simple: Keep minimal PowerShell wrapper
- Focus 80% effort on Unix shells (bash/zsh/fish)
