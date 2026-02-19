#!/usr/bin/env bash

if [[ -n $1 ]]; then
	mkdir $1
	cd $1
	mkdir src

	curl 'https://raw.githubusercontent.com/qweenkie/ggez-template/refs/heads/master/src/main.rs' > src/main.rs
	curl 'https://raw.githubusercontent.com/qweenkie/ggez-template/refs/heads/master/src/gameloop.rs' > src/gameloop.rs

	cargo init
	cargo add ggez --no-default-features
fi
