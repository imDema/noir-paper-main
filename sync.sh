#!/bin/bash
rsync -auvz ${@:2} --filter=':- .gitignore' ./ ${1}:noir-main/ --progress --exclude data/ --exclude results/
rsync -auvz ${@:2} --filter=':- .gitignore' ${1}:noir-main/ ./ --progress --exclude data/
rsync -auvz ${@:2} ${1}:noir-main/results ./ --progress
