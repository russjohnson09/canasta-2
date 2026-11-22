#!/usr/bin/env bash
git fetch origin main || true
git diff origin/main HEAD > diff.patch || git diff HEAD~1 HEAD > diff.patch || touch diff.patch
echo "Generated diff.patch."


