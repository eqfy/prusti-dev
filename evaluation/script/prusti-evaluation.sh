#!/bin/bash

# Run the Prusti evaluation
#
# Usage: script <crate/download/dir>

set -uo pipefail

info() { echo -e "[-] ($(date '+%Y-%m-%d %H:%M:%S')) ${*}"; }
error() { echo -e "[!] ($(date '+%Y-%m-%d %H:%M:%S')) ${*}"; }

# Get the directory in which this script is contained
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

# Get the folder in which all the crates has been downloaded
CRATE_DOWNLOAD_DIR="$(cd "${1:-.}" && pwd)"
cd "$CRATE_DOWNLOAD_DIR"

if [[ ! -d "$CRATE_DOWNLOAD_DIR/000_libc" ]]; then
	error "It looks like CRATE_DOWNLOAD_DIR is wrong: '$CRATE_DOWNLOAD_DIR'"
	exit 1
fi
info "Using CRATE_DOWNLOAD_DIR=$CRATE_DOWNLOAD_DIR"

# Timeout
RUSTC_TIMEOUT="900"
PRUSTI_TIMEOUT="3600"
info "Using RUSTC_TIMEOUT=$RUSTC_TIMEOUT seconds"
info "Using PRUSTI_TIMEOUT=$PRUSTI_TIMEOUT seconds"

# Viper and Z3
export VIPER_HOME="$DIR/../../../viper"
export Z3_EXE="$DIR/../../../z3/bin/z3"

if [ -z "$(ls -A "$VIPER_HOME/"*.jar)" ]; then
	error "It looks like VIPER_HOME is wrong: '$VIPER_HOME'"
	exit 1
fi

if [ ! -x "$Z3_EXE" ]; then
	error "It looks like Z3_EXE is wrong: '$Z3_EXE'"
	exit 1
fi

start_date="$(date '+%Y-%m-%d-%H%M%S')"
evaluation_log_file="$CRATE_DOWNLOAD_DIR/prusti-evaluation-log-$start_date.log"
evaluation_log_file_final="$CRATE_DOWNLOAD_DIR/prusti-evaluation-log.log"
info "Using evaluation_log_file='$evaluation_log_file'"

(
	# Already done
	# "$DIR/compile-crates.sh" "$CRATE_DOWNLOAD_DIR" "$TIMEOUT"

	"$DIR/filter-crates.sh" "$CRATE_DOWNLOAD_DIR" "$CRATE_DOWNLOAD_DIR/supported-crates.csv" "$PRUSTI_TIMEOUT"

	"$DIR/whitelist-crates.sh" "$CRATE_DOWNLOAD_DIR" "$CRATE_DOWNLOAD_DIR/supported-crates.csv"

	PRUSTI_CHECK_PANICS=true PRUSTI_CHECK_BINARY_OPERATIONS=true \
	"$DIR/verify-crates-fine-grained.sh" "$CRATE_DOWNLOAD_DIR" "$CRATE_DOWNLOAD_DIR/supported-crates.csv" \
		"supported-procedures-with-assertions.csv" "$PRUSTI_TIMEOUT"

    # Run 1
	"$DIR/verify-crates-coarse-grained.sh" "$CRATE_DOWNLOAD_DIR" "$CRATE_DOWNLOAD_DIR/supported-crates.csv" \
		"supported-procedures.csv" "$PRUSTI_TIMEOUT"

	# Run 2
	"$DIR/verify-crates-coarse-grained.sh" "$CRATE_DOWNLOAD_DIR" "$CRATE_DOWNLOAD_DIR/supported-crates.csv" \
		"supported-procedures.csv" "$PRUSTI_TIMEOUT"

	# Run 3
	"$DIR/verify-crates-coarse-grained.sh" "$CRATE_DOWNLOAD_DIR" "$CRATE_DOWNLOAD_DIR/supported-crates.csv" \
		"supported-procedures.csv" "$PRUSTI_TIMEOUT"

) 2>&1 | tee "$evaluation_log_file"

cp "$evaluation_log_file" "$evaluation_log_file_final"
