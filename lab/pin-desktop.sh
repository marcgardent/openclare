#!/usr/bin/env bash
# Keeps the desktop off the job processors (lab, SPEC.md §7.5): every process
# of this user still allowed on every processor is pinned to the desktop's;
# what starts later inherits it. The jobs lab pins are left alone.
#
#   lab/pin-desktop.sh 0,4             the desktop on core 0 (processors 0 and 4)
#   lab/pin-desktop.sh --undo 0,4      those processes back on every processor
#   lab/pin-desktop.sh --dry-run 0,4   what would move, nothing moved
#
# Nothing permanent: a new session starts unpinned. No root needed.
set -u

usage() {
    echo "usage: $0 [--undo] [--dry-run] <the desktop's processors, as 0,4>" >&2
    exit 2
}

undo=false
dry=false
while [ $# -gt 1 ]; do
    case $1 in
        --undo) undo=true ;;
        --dry-run) dry=true ;;
        *) usage ;;
    esac
    shift
done
[ $# -eq 1 ] || usage
desktop=$1
online=$(cat /sys/devices/system/cpu/online)

# The processors of a list, one per line: 1-3,5 gives 1 2 3 5.
expand() {
    local part
    for part in ${1//,/ }; do
        if [[ $part == *-* ]]; then seq "${part%-*}" "${part#*-}"; else echo "$part"; fi
    done
}
same() {
    [ "$(expand "$1" | sort -n | tr '\n' ,)" = "$(expand "$2" | sort -n | tr '\n' ,)" ]
}

if $undo; then from=$desktop; to=$online; else from=$online; to=$desktop; fi
moved=0
failed=0
for pid in $(pgrep -u "$(id -u)"); do
    allowed=$(awk '/^Cpus_allowed_list/ { print $2 }' "/proc/$pid/status" 2>/dev/null) || continue
    [ -n "$allowed" ] || continue
    same "$allowed" "$from" || continue
    if $dry; then
        echo "$pid $(cat "/proc/$pid/comm" 2>/dev/null): $allowed -> $to"
        moved=$((moved + 1))
    elif taskset -a -c -p "$to" "$pid" >/dev/null 2>&1; then
        moved=$((moved + 1))
    else
        failed=$((failed + 1))
    fi
done
if $dry; then
    echo "$moved processes would go on the processors $to"
else
    echo "$moved processes on the processors $to"
    [ "$failed" -eq 0 ] || echo "$failed could not be moved (ended meanwhile, or not ours)"
fi
