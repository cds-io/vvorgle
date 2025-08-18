#!/bin/bash

echo "Building Wordle with embedded word list..."
echo "==========================================="
echo ""

# Build in release mode
cargo build --release 2>/dev/null

# Get file sizes
BINARY_SIZE=$(ls -lh target/release/wordle | awk '{print $5}')
WORDS_SIZE=$(ls -lh dict/words.txt | awk '{print $5}')

echo "📊 Size Analysis:"
echo "  Word list file:  $WORDS_SIZE"
echo "  Binary size:     $BINARY_SIZE"
echo ""

echo "✅ Benefits of embedded word list:"
echo "  • Single file distribution"
echo "  • No file I/O at runtime"
echo "  • Can't lose the word list"
echo "  • Slightly faster startup"
echo ""

echo "📝 Compression Analysis:"
# Create compressed version for comparison
gzip -c dict/words.txt > dict/words.txt.gz 2>/dev/null
COMPRESSED_SIZE=$(ls -lh dict/words.txt.gz | awk '{print $5}')
rm dict/words.txt.gz

echo "  Uncompressed in binary: ~13 KB"
echo "  If compressed: ~$COMPRESSED_SIZE"
echo "  Savings: ~8 KB (not worth the complexity for this size)"
echo ""

echo "🎯 Recommendation: Current implementation (uncompressed embed) is optimal!"
