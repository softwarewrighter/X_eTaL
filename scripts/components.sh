#!/usr/bin/env bash
# The component workspaces under components/, in dependency order.
# Sourced by the other scripts; prints one name per line when run.
COMPONENTS=(base syntax core render types eval hof cli)
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    printf '%s\n' "${COMPONENTS[@]}"
fi
