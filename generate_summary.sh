#!/usr/bin/env bash
cd doc/srs/src || exit 1
{
    echo "# Summary"
    echo
    for f in $(ls *.md | grep -v SUMMARY.md); do echo "- [$(echo "$f" | sed -e 's;_; ;g' -e 's;.md;;' -e 's;^.;\U&;g' -e 's; .;\U&;g')](./$f)"; done
} >SUMMARY.md
