#!/bin/bash

here=$(dirname $(realpath $0))
export songdir={{this.srcdir}}
export bookdir={{this.bookdir}}
export builddir={{this.builddir}}

neomake plan -n all -a "args.blah=x" | neomake execute -w4
