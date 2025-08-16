#!/bin/bash

echo "=== Testing Simple Solver ==="
echo -e "1\n1\nCRANE BBBBY\nquit" | cargo run --quiet 2>/dev/null | grep -E "(first guess|next guess):" | head -2

echo -e "\n=== Testing Entropy Solver ==="
echo -e "1\n2\nCRANE BBBBY\nquit" | cargo run --quiet 2>/dev/null | grep -E "(first guess|next guess):" | head -2

echo -e "\n=== Entropy Solver with small candidate pool ==="
echo "Testing with CRANE->127 candidates, then BEEFY->6 candidates"
echo -e "1\n2\nCRANE BBBBY\nBEEFY GGBBB\nquit" | cargo run --quiet 2>/dev/null | grep -E "(Candidates remaining|next guess):" | tail -3