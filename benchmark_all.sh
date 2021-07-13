#!/bin/zsh

hyperfine -w 1 -r 3 $(fd -I -t x "^(\d+)$" | sd "^(.+)$" './${1}')
