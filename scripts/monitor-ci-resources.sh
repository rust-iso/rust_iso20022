#!/usr/bin/env bash
# Measurements survive a failed compiler step and are uploaded by CI.
set -euo pipefail

while true; do
    date -u '+%Y-%m-%dT%H:%M:%SZ'
    free -m
    df -h .
    ps -eo pid,ppid,rss,comm --sort=-rss | head -12 || true
    for events in /sys/fs/cgroup/memory.events /sys/fs/cgroup/memory/memory.failcnt; do
        if [[ -r "$events" ]]; then
            cat "$events"
        fi
    done
    sleep 15
done
