#!/bin/bash

zola build --output-dir ../docs --force
echo "reporg.de" > ../docs/CNAME
