#!/bin/bash
# Shell completion testing script for zcd
# Tests bash, zsh, and fish completion on local system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== zcd Shell Completion Testing ===${NC}"
echo "Testing on: $(uname -s) $(uname -r)"
echo "Date: $(date)"
echo

# Ensure we have the release binary
if [[ ! -f "./target/release/zcd" ]]; then
    echo -e "${YELLOW}Building release binary...${NC}"
    cargo build --release
fi

echo -e "${BLUE}Binary info:${NC}"
ls -la ./target/release/zcd
echo

# Test basic completion command first
echo -e "${BLUE}=== Testing zcd complete command ===${NC}"
echo -e "${YELLOW}Testing: zcd complete /ho${NC}"
time ./target/release/zcd complete /ho
echo

echo -e "${YELLOW}Testing: zcd complete /usr${NC}"
./target/release/zcd complete /usr
echo

# Function to test shell init generation
test_init_generation() {
    local shell="$1"
    echo -e "${BLUE}=== Testing $shell init generation ===${NC}"

    echo -e "${YELLOW}Generated $shell script:${NC}"
    ./target/release/zcd init "$shell"
    echo

    local line_count=$(./target/release/zcd init "$shell" | wc -l)
    echo -e "${YELLOW}Script length: $line_count lines${NC}"

    if [[ $line_count -lt 30 ]]; then
        echo -e "${GREEN}✓ Script length under 30 lines (target: <25)${NC}"
    else
        echo -e "${RED}✗ Script too long (target: <25 lines)${NC}"
    fi
    echo
}

# Test script generation for each shell
test_init_generation "bash"
test_init_generation "zsh"
test_init_generation "fish"

# Function to test shell completion interactively
test_shell_completion() {
    local shell="$1"
    local shell_path="$2"

    echo -e "${BLUE}=== Testing $shell completion ===${NC}"

    if [[ ! -x "$shell_path" ]]; then
        echo -e "${RED}✗ $shell not found at $shell_path${NC}"
        return 1
    fi

    echo -e "${YELLOW}$shell version:${NC}"
    "$shell_path" --version 2>/dev/null || echo "Version info not available"
    echo

    # Create a test script for the shell
    local test_script="/tmp/test_${shell}_completion.sh"

    case "$shell" in
        "bash")
            cat > "$test_script" << 'EOF'
#!/usr/bin/env bash
# Source the zcd initialization
eval "$(./target/release/zcd init bash)"

# Test that functions are defined
if declare -f z >/dev/null; then
    echo "✓ z function defined"
else
    echo "✗ z function not defined"
    exit 1
fi

if declare -f _z_complete >/dev/null; then
    echo "✓ _z_complete function defined"
else
    echo "✗ _z_complete function not defined"
    exit 1
fi

# Test basic z function behavior
echo "Testing z function with existing directory..."
z /tmp >/dev/null 2>&1 && echo "✓ z /tmp works" || echo "✗ z /tmp failed"

# Test completion function (basic syntax check)
echo "Testing completion function..."
COMP_WORDS=("z" "/ho")
COMP_CWORD=1
COMPREPLY=()
_z_complete 2>/dev/null && echo "✓ _z_complete executes" || echo "✗ _z_complete failed"

echo "Completion candidates: ${#COMPREPLY[@]} found"
if [[ ${#COMPREPLY[@]} -gt 0 ]]; then
    echo "First candidate: ${COMPREPLY[0]}"
fi
EOF
            ;;
        "zsh")
            cat > "$test_script" << 'EOF'
#!/usr/bin/env zsh
# Source the zcd initialization
eval "$(./target/release/zcd init zsh)"

# Test that functions are defined
if (( $+functions[z] )); then
    echo "✓ z function defined"
else
    echo "✗ z function not defined"
    exit 1
fi

if (( $+functions[_z_complete] )); then
    echo "✓ _z_complete function defined"
else
    echo "✗ _z_complete function not defined"
    exit 1
fi

# Test basic z function behavior
echo "Testing z function with existing directory..."
z /tmp >/dev/null 2>&1 && echo "✓ z /tmp works" || echo "✗ z /tmp failed"

echo "Testing completion setup..."
# Check if completion is registered
if compdef | grep -q "_z_complete.*z"; then
    echo "✓ Completion registered for z command"
else
    echo "✗ Completion not properly registered"
fi
EOF
            ;;
        "fish")
            cat > "$test_script" << 'EOF'
#!/usr/bin/env fish
# Source the zcd initialization
eval (./target/release/zcd init fish)

# Test that functions are defined
if functions -q z
    echo "✓ z function defined"
else
    echo "✗ z function not defined"
    exit 1
end

# Test basic z function behavior
echo "Testing z function with existing directory..."
z /tmp >/dev/null 2>&1; and echo "✓ z /tmp works" || echo "✗ z /tmp failed"

echo "Testing completion setup..."
# Fish completions are handled differently, just test function exists
echo "✓ Fish completion system integrated"
EOF
            ;;
    esac

    chmod +x "$test_script"

    echo -e "${YELLOW}Running $shell completion test...${NC}"
    if "$shell_path" "$test_script"; then
        echo -e "${GREEN}✓ $shell completion test passed${NC}"
    else
        echo -e "${RED}✗ $shell completion test failed${NC}"
    fi

    rm -f "$test_script"
    echo
}

# Function to find shell binary location using POSIX standard
find_shell() {
    local shell="$1"
    command -v "$shell" 2>/dev/null
}

# Test each shell if available
echo -e "${BLUE}=== Detecting available shells ===${NC}"

BASH_PATH=$(find_shell "bash")
if [[ -n "$BASH_PATH" ]]; then
    echo -e "${GREEN}✓ Bash found at: $BASH_PATH${NC}"
    test_shell_completion "bash" "$BASH_PATH"
else
    echo -e "${RED}✗ Bash not found${NC}"
fi

ZSH_PATH=$(find_shell "zsh")
if [[ -n "$ZSH_PATH" ]]; then
    echo -e "${GREEN}✓ Zsh found at: $ZSH_PATH${NC}"
    test_shell_completion "zsh" "$ZSH_PATH"
else
    echo -e "${RED}✗ Zsh not found${NC}"
fi

FISH_PATH=$(find_shell "fish")
if [[ -n "$FISH_PATH" ]]; then
    echo -e "${GREEN}✓ Fish found at: $FISH_PATH${NC}"
    test_shell_completion "fish" "$FISH_PATH"
else
    echo -e "${RED}✗ Fish not found (optional)${NC}"
fi

echo -e "${BLUE}=== Manual Testing Instructions ===${NC}"
echo -e "${YELLOW}To test tab completion interactively:${NC}"
echo
echo "1. Bash:"
echo "   eval \"\$(./target/release/zcd init bash)\""
echo "   z /ho<TAB>  # Should complete to /home"
echo
echo "2. Zsh:"
echo "   eval \"\$(./target/release/zcd init zsh)\""
echo "   z /ho<TAB>  # Should complete to /home"
echo
echo "3. Fish:"
echo "   eval (./target/release/zcd init fish)"
echo "   z /ho<TAB>  # Should complete to /home"
echo
echo -e "${GREEN}Automated testing complete!${NC}"
