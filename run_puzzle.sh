#!/bin/bash

# This script
#   1. takes a puzzle no (eg. 01..25) as input
#   2. compiles the sources in relevant source directory
#   3. retrieves the puzzle input from http://adventofcode.com/2017
#   4. runs the binaries with the retrieved input

# REQUIRED:
#  1. AOC_SESSION environment variable needs to be set to session cookie value to
#     retrieve the input for the user
#  2. curl & rustc

set -eo pipefail

# set and check puzzle no and directory
DIR=$(dirname ${0})
PUZZLE_NO=${1}

if [ ! -d "${DIR}/src/${PUZZLE_NO}" ]; then
    echo "[*] ${DIR}/src/${PUZZLE_NO} is not a valid puzzle_no/directory"
    exit 1
fi

# check environment variable for cookie
if [ -z "${AOC_SESSION}" ]; then
    echo "[*] AOC_SESSION environment variable needs to be set with AOC session cookie value"
    exit 1
fi

# get and check input
echo "[*] Getting input for PUZZLE_NO=${PUZZLE_NO}"
INPUT=$(curl -s --fail --cookie "session=${AOC_SESSION}" "http://adventofcode.com/2017/day/$((${PUZZLE_NO}))/input")
if [ -z "${INPUT}" ]; then
    echo "[*] Retrieved input is empty"
    exit 1
fi

if [ $((${#INPUT})) -lt 9 ]; then
    INPUT_PRINT="${INPUT}"
else
    INPUT_PRINT="${INPUT:0:8}..."
fi

# create the target dir for binaries
mkdir -p ${DIR}/target/${PUZZLE_NO}

# compile
for source_file in `ls -1 ${DIR}/src/${PUZZLE_NO}/*.rs`; do
    bin_file=$(echo $source_file | sed 's/\(.*\)src\(.*\)\.rs/\1target\2/')
    echo "[*] Compiling ${source_file} > ${bin_file}"
    rustc ${source_file} -o ${bin_file}
done

# run binaries
for bin in `ls -1 ${DIR}/target/${PUZZLE_NO}`; do
    bin_file="${DIR}/target/${PUZZLE_NO}/${bin}"
    echo "[*] Running ${bin_file} with acquired input \"${INPUT_PRINT}[len: ${#INPUT}]\", Out:"
    ${bin_file} "${INPUT}"
done

echo "[*] Done"
