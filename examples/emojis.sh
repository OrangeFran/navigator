#!/bin/bash

# get all emojis
emojify --list | tail -n +6 | sed 's/^ *//g' | navi | sed -e "s/:.*//g" | tr -d ' ' | tr -d '\n' | pbcopy
