#!/bin/bash
while event=$(grep -m1 '^\[ \]' research-plan.md | sed 's/\[ \] //' | tr '-' '_'); [ -n "$event" ]; do
  claude -p "/event-research-update $event"
done
echo "All events completed."
