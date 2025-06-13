#!/usr/bin/env python3
"""Convert hex string file to binary file."""

import sys

if len(sys.argv) != 3:
    print("Usage: hex2bin.py <input_hex_file> <output_bin_file>")
    sys.exit(1)

hex_file = sys.argv[1]
bin_file = sys.argv[2]

with open(hex_file, 'r') as f:
    hex_data = f.read().strip()

# Convert hex string to bytes
binary_data = bytes.fromhex(hex_data)

with open(bin_file, 'wb') as f:
    f.write(binary_data)

print(f"Converted {hex_file} to {bin_file}") 