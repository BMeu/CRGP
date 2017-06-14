#!/bin/bash

# Copyright 2017 Bastian Meyer
#
# Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
# MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
# modified, or distributed except according to those terms.

# Mutually compares all cascade result files in sub-folders.

results=($(find . -name "cascs-*" -print))
output_dir="comparisons"
mkdir -p ${output_dir}
for result1 in ${results[*]}
do
	for result2 in ${results[*]}
	do
		if [[ "${result1}" = "${result2}" ]]
		then
			continue
		fi

		IFS='/' read -ra path1 <<< "${result1}"
		IFS='/' read -ra path2 <<< "${result2}"
		folder1=${path1[1]}
		folder2=${path2[1]}
		output_file="${output_dir}/${folder1}_${folder2}.diff"

		eval "./compare_outputs.sh ${result1} ${result2} ${output_file}"
	done
done
