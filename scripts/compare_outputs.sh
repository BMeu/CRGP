#!/bin/bash

# Copyright 2017 Bastian Meyer
#
# Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
# MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
# modified, or distributed except according to those terms.

# Compares the two input files line by line and writes the differences to the output file.

if [ $# -ne 3 ]
    then
        echo "USAGE: $0 [INPUT FILE 1] [INPUT FILE 2] [OUTPUT FILE]"
        echo ""
        echo "Compares the two input files line by line and writes the differences to the output file."
        exit 1
fi

comm -3 <(sort $1) <(sort $2) > $3
