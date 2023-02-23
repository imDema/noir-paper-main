#!/bin/bash
rsync -auvz ${@:2} --filter=':- .gitignore' ./ ${1}:noir-main/ --progress --exclude data --exclude data/
rsync -auvz ${@:2} --filter=':- .gitignore' ${1}:noir-main/ ./ --progress --exclude data --exclude data/
