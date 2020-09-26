#!/bin/sh

# Sorry for the long line.
# Improvements are always welcome!

# requires python
# find . -not -path '*/\.*' | python -c "import sys as s;s.a=[];[setattr(s,'a',list(filter(lambda p: c.startswith(p+'/'),s.a)))or (s.stdout.write('\t'*len(s.a)+c[len(s.a[-1])+1 if s.a else 0:])or True) and s.a.append(c[:-1]) for c in s.stdin]" | navigator

# only-shell version
find . | tail +2 | sed -e "s/\.\///g" -e "s/[^-][^\/]*\//\\t/g" | navigator
