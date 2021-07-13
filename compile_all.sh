#!/bin/zsh

for i in *.rs; do
	echo $i && rustc -C debuginfo=0 -C opt-level=3 -C target-cpu=native -C panic=abort $i
done
