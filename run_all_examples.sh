#!/bin/bash

##
## Rudimentary script to run all examples in the project
## Just a convenience to save me som typing
##

##
# Shell color definitions
RED='\033[0;31m'
PURPLE='\033[0;34m'
NC='\033[0m' # No Color

##
# Print a nice section header for each sample beeing run
function sample_header {
    echo -------------------------------------
    printf "${PURPLE}$1${NC}\n"
    echo -------------------------------------
}

sample_header "Create a UberByte"
cargo run --package uberbyte --example create

sample_header simple
cargo run --package uberbyte --example is_set 

sample_header set
cargo run --package uberbyte --example set

sample_header "check mask set"
cargo run --package uberbyte --example check_mask_set

sample_header "shift"
cargo run --package uberbyte --example shift

sample_header "Formatters"
cargo run --package uberbyte --example formatters