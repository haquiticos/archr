#!/usr/bin/env python3
"""Test script to call archr validate directly via subprocess"""

import subprocess
import json

result = subprocess.run(
    ['python3', 'skill/scripts/archr.py', 'validate', 'model.yaml'],
    capture_output=True,
    text=True
)

print("Exit code:", result.returncode)
print("STDOUT:")
print(result.stdout)
if result.stderr:
    print("STDERR:")
    print(result.stderr)
