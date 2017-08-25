#!/bin/sh

ng build -prod && \
	rsync -avz --delete -del -e ssh ./dist/ ellwein.net:/var/www/beercalc/

