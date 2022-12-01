#!/bin/sh

if [ -z "$1" ]; then
  echo "Please provide the day to download as an argument."
  exit 1
fi

xdg-open "https://adventofcode.com/2022/day/$1/input"

if [ "$?" != "0" ]; then
  echo "Could not download day \"$1\", please use a number 1-25."
  exit 1
fi
