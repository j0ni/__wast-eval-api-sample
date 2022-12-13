#!/usr/bin/env fish
jq --arg text (tr -d '\n' < add_one.wast ) '.wast = $text' wast.json
