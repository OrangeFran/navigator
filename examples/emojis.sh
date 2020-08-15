#!/bin/bash

# get all emojis
emojify --list | tail -n +6 | navigator --stdin 2> /tmp/output

cat /tmp/output | sed -e "s/\s*//g" -e "s/:.*//g" | pbcopy
