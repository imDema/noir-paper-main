#!/bin/bash
cloc --strip-comments=nc --original-dir ./*/
fd -I -E=nc/ -tf '\.nc' -x bash -c 'mkdir -p nc/{//} && mv {} ./nc/{}'
cd nc
fd -I -td --max-depth 3 --min-depth 3 -j1 -x bash -c 'echo $(echo {} | sed "s@\./@@g" | sed "s@/@,@g"),$(tar c {} | gzip -9 | wc -c)' >> ../gzipped.csv
cd ..
sort gzipped.csv -h -o gzipped.csv
echo -e "set,experiment,system,size\n$(cat gzipped.csv)" > gzipped.csv
