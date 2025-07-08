#!/bin/sh

## ------------------------------------------------------
## This requires Tailwind CSS v4.1 CLI installed locally.
## If this is not installed, see /readme.md for details.
## ------------------------------------------------------

npx tailwindcss -i ./src/ui/styles/styles.css -o ./assets/css/main.css --watch

