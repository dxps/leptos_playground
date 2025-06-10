#!/bin/sh

## ------------------------------------------------------
## This requires Tailwind CSS v4.1 CLI installed locally.
## If this is not the case, run:
## npm install tailwindcss@4.1.8 @tailwindcss/cli@4.1.8
## ------------------------------------------------------

npx tailwindcss -i ./style.css -o ./assets/css/main.css --watch

