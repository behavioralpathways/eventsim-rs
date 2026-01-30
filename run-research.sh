#!/bin/bash
while event=$(grep -m1 '^\[ \]' research-plan.md | sed 's/\[ \] //'); [ -n "$event" ]; do
  echo "Processing: $event"
  claude -p "/event-research-update $(echo $event | tr '-' '_')"
  sed -i '' "s/\[ \] $event/[x] $event/" research-plan.md
  echo "Completed: $event"
done
echo "All events completed."
