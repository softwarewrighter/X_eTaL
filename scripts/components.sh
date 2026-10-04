#!/usr/bin/env bash
# The component workspaces under components/, in dependency order.
# Sourced by the other scripts; prints one name per line when run.
COMPONENTS=(base syntax expand lookup macro core render view types eval hof search radix draw system axes step console tui line cli web)
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    printf '%s\n' "${COMPONENTS[@]}"
fi
