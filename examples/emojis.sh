#!/bin/bash

# get all emojis
emojify --list | tail -n +6 | navigator 2> /tmp/output

cat /tmp/output | sed -e "s/:.*//g" | tr -d ' ' | tr -d '\n' | pbcopy