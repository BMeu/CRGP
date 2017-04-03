#!/bin/bash
# Compares the two input files line by line and writes the differences to the output file.

if [ $# -ne 3 ]
    then
        echo "USAGE: $0 [INPUT FILE 1] [INPUT FILE 2] [OUTPUT FILE]"
        echo ""
        echo "Compares the two input files line by line and writes the differences to the output file."
        exit 1
fi

comm -3 <(sort $1) <(sort $2) > $3
