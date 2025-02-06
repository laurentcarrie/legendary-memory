#!/bin/sh

set -e

other=dedix/codespace-work
current=work

command="git diff $current $other --no-renames --name-only "


echo  "added : " $($command --diff-filter A | wc --lines )
echo  "deleted : " $($command --diff-filter D | wc --lines )
echo  "modified : " $($command --diff-filter M | wc --lines )
echo  "copied : " $($command --diff-filter C | wc --lines )
echo  "renamed : " $($command --diff-filter R | wc --lines )

git diff $current $other --name-only --diff-filter A | while read f ; do
mkdir -p $(dirname $f)
git show $other:$f > $f
git add $f
done

git diff $current $other --name-only --diff-filter M | while read f ; do
git show $other:$f > $f
done

git diff $current $other --name-only --diff-filter D | while read f ; do
rm -rf $f
done
