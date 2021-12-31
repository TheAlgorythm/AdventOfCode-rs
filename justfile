set shell := ["zsh", "-cu"]

alias b := build
alias ba := build-all
alias c := clippy
alias ca := clippy-all
alias r := run
alias ra := run-all
alias t := test
alias ta := test-all

default:
  @just --list

build MAIN:
  rustc -C debuginfo=0 -C opt-level=3 -C target-cpu=native -C panic=abort {{MAIN}}.rs

build-all:
  fd -I "^(\d+).rs$" | sd "^(\d+).rs$" 'just build ${1}' | parallel

clean-all:
  for i in $(fd -I -t x "^(\d+)$"); do rm $i; done
  for i in $(fd -I -t x "^(\d+)_test$"); do rm $i; done

clippy MAIN:
  clippy-driver -C debuginfo=0 -C opt-level=3 -C target-cpu=native -C panic=abort {{MAIN}}.rs

clippy-all:
  for i in $(fd -I "^(\d+).rs$" | sd "^(\d+).rs$" '${1}'); do just clippy $i; done

run MAIN:
  ulimit -s 65532
  ./{{MAIN}}

run-all:
  ulimit -s 65532
  for i in $(fd -I -t x "^(\d+)$"); do echo ${i}: && ./$i; done

test MAIN:
  rustc -C debuginfo=0 -C opt-level=3 -C target-cpu=native --test {{MAIN}}.rs -o {{MAIN}}_test 2>/dev/null && ./{{MAIN}}_test

test-all:
  fd -I "^(\d+).rs$" | sd "^(\d+).rs$" 'just test ${1}' | parallel

bench MAIN:
  ulimit -s 65532
  hyperfine -w 2 -r 5 ./{{MAIN}}

bench-all:
  ulimit -s 65532
  hyperfine -w 1 -r 3 $(fd -I -t x "^(\d+)$" | sd "^(.+)$" './${1}')
