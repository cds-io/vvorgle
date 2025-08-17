#!/bin/sh
#
# Setup script to install git hooks
#

echo "Setting up git hooks..."

# Get the git directory (usually .git, but could be different in worktrees)
GIT_DIR=$(git rev-parse --git-dir 2>/dev/null)

if [ $? -ne 0 ]; then
    echo "Error: Not in a git repository"
    exit 1
fi

HOOKS_DIR="${GIT_DIR}/hooks"

# Create hooks directory if it doesn't exist
mkdir -p "$HOOKS_DIR"

# Install pre-commit hook
if [ -f ".githooks/pre-commit" ]; then
    echo "Installing pre-commit hook..."
    
    # Remove existing hook if it exists (could be a file or symlink)
    if [ -e "${HOOKS_DIR}/pre-commit" ]; then
        rm "${HOOKS_DIR}/pre-commit"
    fi
    
    # Create symlink to our hook
    ln -s "../../.githooks/pre-commit" "${HOOKS_DIR}/pre-commit"
    
    if [ $? -eq 0 ]; then
        echo "✓ Pre-commit hook installed successfully"
    else
        echo "Failed to install pre-commit hook"
        exit 1
    fi
else
    echo "Warning: .githooks/pre-commit not found"
fi

echo ""
echo "Git hooks setup complete!"
echo "The pre-commit hook will run 'cargo fmt' automatically before each commit."
echo ""
echo "To bypass the hook for a single commit, use: git commit --no-verify"