#!/bin/sh

BASEDIR=$(dirname $0)

DATABASE_URL=postgresql://user_dir_lap:user_dir_lap@localhost:5450/user_dir_lap

sqlx migrate run --source $BASEDIR/migrations --database-url $DATABASE_URL
