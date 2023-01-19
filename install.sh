#!/usr/bin/env bash

version=0.2.3

if [ "$(uname)" == "Darwin" ]; then
	if [ "$(uname -m)" == "x86" ]; then
		sudo curl -fsSL https://github.com/Robert-Cunningham/yeet-rs/releases/download/v${version}/yeet-darwin-x64 > /usr/local/bin/yeet
		chmod +x /usr/local/bin/yeet
	elif [ "$(uname -m)" == "arm64" ]; then
		sudo curl -fsSL https://github.com/Robert-Cunningham/yeet-rs/releases/download/v${version}/yeet-darwin-aarch64 > /usr/local/bin/yeet
		chmod +x /usr/local/bin/yeet
	fi
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
	sudo curl -fsSL https://github.com/Robert-Cunningham/yeet-rs/releases/download/v${version}/yeet-linux-x64 > /usr/local/bin/yeet
	chmod +x /usr/local/bin/yeet
elif [ "$(expr substr $(uname -s) 1 10)" == "MINGW64_NT" ]; then
	echo "yeet-rs does not support windows."
fi
