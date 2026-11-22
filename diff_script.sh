#!/usr/bin/env bash
git diff origin/main HEAD > diff.patch
echo "Generated diff.patch between origin/main and HEAD."

