#!/bin/bash

all() {
    http get localhost:8080/
}

add() {
    http post localhost:8080/ title="new todo2"
}

update() {
    http put localhost:8080/1 --raw '{ "title": "write Rust updated", "completed": true }'
}

delete() {
    http delete localhost:8080/1
}

read -p "type operator (all, add, update, delete): " list

for name in $list; do
    $name
done
