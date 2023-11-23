#!/bin/bash
exclusions=(
  "target"
  ".github"
  ".idea"
  "ocean-language-support/intellij/ocean/.gradle"
  "ocean-language-support/intellij/ocean/.idea"
  "ocean-language-support/intellij/ocean/.run"
  "ocean-language-support/intellij/ocean/build"
  "ocean-language-support/intellij/ocean/gradle"
)

cloc --fullpath --not-match-d="($(IFS='|' ; echo "${exclusions[*]}"))" .