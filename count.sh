#!/bin/sh

echo "Lines"
find src -name '*.rs' | xargs wc -l | sort -nr
echo "Words"
find src -name '*.rs' | xargs wc -w | sort -nr
echo "Characters"
find src -name '*.rs' | xargs wc -m | sort -nr