#!/bin/bash
DIR="$( printf "%s%02d" day $1)"
[ -d $DIR ] && echo "Directory already exists" && exit
cargo new $DIR
touch $DIR/input.txt
touch $DIR/minput.txt
cp template.rs $DIR/src/main.rs