#!/usr/bin/env bash

# Copyright 2017 Bastian Meyer
#
# Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
# MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
# modified, or distributed except according to those terms.

# Compile several branches, then run their executables with several worker configuration for ten times each.

branches=(alg-var-adaptive-hashset alg-var-user-hashset alg-var-prefix-hashset alg-var-adaptive-vec-sorted alg-var-user-vec-sorted alg-var-prefix-vec-sorted alg-var-adaptive-vec-unsorted alg-var-user-vec-unsorted alg-var-prefix-vec-unsorted)
executables=(hashset-adaptive hashset-user hashset-prefix vec-sorted-adaptive vec-sorted-user vec-sorted-prefix vec-unsorted-adaptive vec-unsorted-user vec-unsorted-prefix)
workers=4
runs=10

for index in ${!branches[*]}
do
    branch=${branches[$index]}
    executable=${executables[$index]}
    git checkout ${branch}
    cargo build --release
    mv target/release/crgp ${executable}
done

for run in `seq 1 ${runs}`
do
    for worker in `seq 1 ${workers}`
    do
        for executable in ${executables[*]}
        do
            output="results/${executable}-${worker}"
            mkdir -p ${output}
            command="./${executable} -w ${worker} -o ${output} ~/socialgraph/friends-tar ~/cascades/medium.json"
            eval ${command}
        done
    done
done
