#!/bin/sh

yarn build && \
	rsync -avz --delete -del -e ssh ./build/ ellwein.net:/var/www/beercalc/

