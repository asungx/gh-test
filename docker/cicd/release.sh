#!/bin/bash

# See `set --help`
set -o nounset

# Style settings
Bold='\033[1m'
Underline='\033[4m'

# Bold colors
BRed='\033[1;31m'
BGreen='\033[1;32m'
BWhite='\033[1;37m'

# No Color
NC='\033[0m'

function err() {
    printf "${BRed}Error${NC}: $1\n" >&2
    exit 1
}

function info() {
    printf "${BGreen}Info${NC}: $1\n" >&1
}

# Run a command that should never fail. If the command fails execution
# will immediately terminate with an error showing the failing
# command.
function ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

# Prints arg help menu
function display_help() {
    # TODO: Implement
    :
}

function get_args() {
    # TODO: Implement with getopt
    :
}

function check_argument_variables() {
    # TODO: Implement
    :
}

function main() {
    get_args

    check_argument_variables
}

main "$@" || exit 1
