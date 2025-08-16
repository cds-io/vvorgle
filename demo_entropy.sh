#!/bin/bash

echo "Testing Entropy Solver for solution BEVEL"
echo "======================================="
echo ""
echo "Simulating gameplay with Entropy strategy:"
echo ""

# Use entropy solver (option 2)
# Simulate: SALET -> BBBBY (only E matches)
# Then the solver should suggest a good elimination word
echo -e "1\n2\nSALET BBBBY\nCRANE BBBBY\nBEEFY GGBBB\nBEVEL GGGGG\n" | cargo run --quiet 2>/dev/null | grep -E "(Suggested|Candidates remaining|feedback:|possibility)" | head -20