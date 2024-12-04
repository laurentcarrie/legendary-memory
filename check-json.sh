#!/bin/bash

set -e
#set -x
here=$(dirname $(realpath $0))
source $here/software/others/shfiles/colors.sh

work() {
	jsonfile=$1
	# printf "${Green}$jsonfile${Color_Off}\n"
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

git ls-files | grep ".json$" | while read f ; do work $f ; done