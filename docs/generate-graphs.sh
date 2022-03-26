#!/bin/sh

for file in **.dot; do
    dot -Tsvg -O ${file}
done
