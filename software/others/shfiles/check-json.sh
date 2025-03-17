#!/bin/bash

set -e
#set -x
inputdir=$1
test "x$inputdir"!="x"
here=$(dirname $(realpath $0))
root=$(dirname $(dirname $here))
source $here/colors.sh

work() {
	jsonfile=$1
	#printf "${Green}$jsonfile${Color_Off}\n"
	tmpresultfile=$(mktemp /tmp/pch-legendary-memory.XXXXXX)
	(
		jq "."  $jsonfile > $tmpresultfile  ||
	(
		printf "Error : ${Red}$jsonfile${Color_Off}\n" ;
		jq "."  $jsonfile
	))
	( diff $jsonfile $tmpresultfile > /dev/null )  || (
		printf "changed : ${Red}$jsonfile${Color_Off}\n"
		cp $tmpresultfile $jsonfile
	)

}

# git ls-files | grep ".json$" | while read f ; do work $root/$f ; done

find $inputdir -type f -name "*.json" | while read f ; do work $f ; done
