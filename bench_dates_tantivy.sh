#!/bin/bash
#!/usr/bin/env bash

PATH=$PATH:$PWD

# get command line parameters
# usage: ./bench_dates -s 2022-08-01 -e 2022-09-01
# enddate is optional, it defaults to today
while getopts s:e: flag
do
    case "${flag}" in
        s) input_start_date=${OPTARG};;
        e) input_end_date=${OPTARG};;
        *) exit
    esac
done

startdate=$(date -I -d "$input_start_date") || exit 
enddate=$(date -I -d "$input_end_date")     || exit
echo "Startdate $startdate Enddate $enddate"

enddate=$(date -I -d "$input_end_date + 1 day")     || exit  # +1 to make the range inclusive
d="$startdate"
while [ "$d" != "$enddate" ]; do 
  echo "$d"
  d=$(date -I -d "$d + 1 day")
  ./bench_tantivy.sh -d "$d";
done
