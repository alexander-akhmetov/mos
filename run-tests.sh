#!/bin/bash

for file in src/bin/*
do
    filename=$(basename -- "$file")
    test_name=${filename%.rs}
    make integration-test/$test_name
done
