#!/usr/bin/env bash

branches=(alg-var-adaptive-hashset alg-var-user-hashset alg-var-prefix-hashset alg-var-adaptive-vec-sorted alg-var-user-vec-sorted alg-var-prefix-vec-sorted alg-var-adaptive-vec-unsorted alg-var-user-vec-unsorted alg-var-prefix-vec-unsorted)
execs=(hashset-adaptive hashset-user hashset-prefix vec-sorted-adaptive vec-sorted-user vec-sorted-prefix vec-unsorted-adaptive vec-unsorted-user vec-unsorted-prefix)
workers=4
runs=10

for index in ${!branches[*]}
do
    branch=${branches[$index]}
    name=${execs[$index]}
    git checkout ${branch}
    cargo build --release
    mv target/release/crgp ${name}
done

for exe in ${execs[*]}
do
    for w in `seq 1 ${workers}`
    do
        for r in `seq 1 ${runs}`
        do
            path="results/${exe}-${w}"
            command="./${exe} -w ${w} -o ${path} ~/socialgraph/friends-tar ~/cascades/6-100\n"
            eval ${command}
        done
    done
done
