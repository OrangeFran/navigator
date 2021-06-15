#!/usr/bin/env zsh

# Wrapper around the cli to overcome usability issues
# because of the stdin/stdout redirecting and the terminal user interface
# which can somehow not be displayed at the same time
# (hopefully a temporary workaround)

# How this works:
# It swaps standard out and standard
# error so only stderr get's outputted to the next command
navigator $@ 3>&2 2>&1 1>&3 3>&-
