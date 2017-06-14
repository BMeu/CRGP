#!/usr/bin/env bash

# Copyright 2017 Bastian Meyer
#
# Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
# MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
# modified, or distributed except according to those terms.

# Compile CRGP, then run it in all possible distribution settings on a single machine.

cores=10
runs=10

cargo build --release
for run in `seq 1 ${runs}`
do
    for processes in `seq 1 ${cores}`
    do
        for workers in `seq 1 ${cores}`
        do
            threads=$((${processes} * ${workers}))
            if [[ "${threads}" -gt "${cores}" ]]; then
                continue
            fi

            output="results/p${processes}-w${workers}"
            mkdir -p ${output}
            command_base="./target/release/crgp -w ${workers} -n ${processes} -o ${output} --pad-users ~/socialgraph/friends-tar ~/cascades/medium.json"
            command=""
            for process in `seq 0 $((${processes} - 1))`
            do
                command="${command}${command_base} -p ${process} & "
            done
            eval "${command} wait"
        done
    done
done
