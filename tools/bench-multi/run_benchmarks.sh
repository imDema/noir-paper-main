#!/usr/bin/env bash

NOIR_HOME=/home/ubuntu/noir-main

set -xe

# ./bench-multi.py $@ --capture-output q0-new timely ${NOIR_HOME}/bench/timely/nexmark timely -- --migration none --duration 10 --rate 10000000 --queries q0
# ./bench-multi.py $@ --capture-output q1-new timely ${NOIR_HOME}/bench/timely/nexmark timely -- --migration none --duration 10 --rate 10000000 --queries q1
# ./bench-multi.py $@ --capture-output q2-new timely ${NOIR_HOME}/bench/timely/nexmark timely -- --migration none --duration 10 --rate 10000000 --queries q2
# ./bench-multi.py $@ --capture-output q3-new timely ${NOIR_HOME}/bench/timely/nexmark timely -- --migration none --duration 10 --rate 10000000 --queries q3
# ./bench-multi.py $@ --capture-output q4-new timely ${NOIR_HOME}/bench/timely/nexmark timely -- --migration none --duration 10 --rate 10000000 --queries q4
# ./bench-multi.py $@ --capture-output q5-new timely ${NOIR_HOME}/bench/timely/nexmark timely -- --migration none --duration 10 --rate 10000000 --queries q5
# ./bench-multi.py $@ --capture-output q6-new timely ${NOIR_HOME}/bench/timely/nexmark timely -- --migration none --duration 10 --rate 10000000 --queries q6
# ./bench-multi.py $@ --capture-output q7-new timely ${NOIR_HOME}/bench/timely/nexmark timely -- --migration none --duration 10 --rate 10000000 --queries q7
# ./bench-multi.py $@ --capture-output q8-new timely ${NOIR_HOME}/bench/timely/nexmark timely -- --migration none --duration 10 --rate 10000000 --queries q8

# ./bench-multi.py $@ --alias noir-tcp-tls --capture-output q0 noir ${NOIR_HOME}/noir nexmark -- 100000000 0
# ./bench-multi.py $@ --alias noir-tcp-tls --capture-output q1 noir ${NOIR_HOME}/noir nexmark -- 100000000 1
# ./bench-multi.py $@ --alias noir-tcp-tls --capture-output q2 noir ${NOIR_HOME}/noir nexmark -- 100000000 2
./bench-multi.py $@ --alias noir-tcp-tls q3 noir ${NOIR_HOME}/noir nexmark -- 5000000 3
./bench-multi.py $@ --alias noir-tcp-tls --capture-output q4 noir ${NOIR_HOME}/noir nexmark -- 5000000 4
./bench-multi.py $@ --alias noir-tcp-tls --capture-output q5 noir ${NOIR_HOME}/noir nexmark -- 5000000 5
./bench-multi.py $@ --alias noir-tcp-tls --capture-output q6 noir ${NOIR_HOME}/noir nexmark -- 5000000 6
./bench-multi.py $@ --alias noir-tcp-tls --capture-output q7 noir ${NOIR_HOME}/noir nexmark -- 5000000 7
./bench-multi.py $@ --alias noir-tcp-tls --capture-output q8 noir ${NOIR_HOME}/noir nexmark -- 5000000 8

# ./bench-multi.py $@ q2-lat noir ${NOIR_HOME}/noir nexmark-latency -- 100000000 2 --batch 32768 --dt-us 1000
# ./bench-multi.py $@ q3-lat noir ${NOIR_HOME}/noir nexmark-latency -- 100000000 3 --batch 32768 --dt-us 1000
# ./bench-multi.py $@ q5-lat noir ${NOIR_HOME}/noir nexmark-latency -- 100000000 5 --batch 32768 --dt-us 1000

# ./bench-multi.py $@ q2-lat timely ${NOIR_HOME}/bench/timely/nexmark-latency timely -- --migration none --duration 10 --rate 10000000 --queries q2
# ./bench-multi.py $@ q3-lat timely ${NOIR_HOME}/bench/timely/nexmark-latency timely -- --migration none --duration 10 --rate 10000000 --queries q3
# ./bench-multi.py $@ q5-lat timely ${NOIR_HOME}/bench/timely/nexmark-latency timely -- --migration none --duration 10 --rate 10000000 --queries q5

# ./bench-multi.py $@ pagerank noir ${NOIR_HOME}/noir pagerank -- 100 81306 ${NOIR_HOME}/data/pagerank/nodes.txt ${NOIR_HOME}/data/pagerank/edges.txt
# ./bench-multi.py $@ pagerank flink ${NOIR_HOME}/bench/flink/PageRank pagerank-1.0.jar -- -iterations 100 -numPages 81306 -pages ${NOIR_HOME}/data/pagerank/nodes.txt -links ${NOIR_HOME}/data/pagerank/edges.txt

# ./bench-multi.py $@ pagerank-timely noir ${NOIR_HOME}/bench/noir-extra noir-pagerank-timely -- 100 80000 2500000
# ./bench-multi.py $@ --alias noir-ad-hoc pagerank-timely noir ${NOIR_HOME}/bench/noir-extra noir-pagerank-delta -- 100 80000 2500000
# ./bench-multi.py $@ pagerank-timely timely ${NOIR_HOME}/bench/timely differential-pagerank -- 100 80000 2500000

# ./bench-multi.py $@ pagerank-mpi noir ${NOIR_HOME}/noir pagerank_stateful -- 100 81306 ${NOIR_HOME}/data/pagerank/nodes.txt ${NOIR_HOME}/data/pagerank/edges.txt
# ./bench-multi.py $@ pagerank-mpi mpi ${NOIR_HOME}/bench/mpi/PageRank main -- -i 100 -n 81306 -d ${NOIR_HOME}/data/pagerank/edges.txt


# ./bench-multi.py $@ wordcount noir ${NOIR_HOME}/noir wordcount -- ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt
# ./bench-multi.py $@ wordcount timely ${NOIR_HOME}/bench/timely timely-wordcount -- ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt

# ./bench-multi.py $@ --alias mpi-mmap wordcount-opt mpi ${NOIR_HOME}/bench/mpi/wordcount main -- -d ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt -m mmap -T 1
# ./bench-multi.py $@ wordcount-opt mpi ${NOIR_HOME}/bench/mpi/wordcount main -- -d ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt -m iostream_opt -T 1
./bench-multi.py $@ --alias noir-tcp-tls wordcount-opt noir ${NOIR_HOME}/noir wordcount_opt -- ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt
./bench-multi.py $@ --alias noir-tcp-tls wordcount-re noir ${NOIR_HOME}/noir wordcount_assoc -- ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt
./bench-multi.py $@ --alias noir-tcp-tls wordcount-nonassoc noir ${NOIR_HOME}/noir wordcount -- ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt
# ./bench-multi.py $@ wordcount-re mpi ${NOIR_HOME}/bench/mpi/wordcount main -- -d ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt -m iostream -T 1
# ./bench-multi.py $@ wordcount-re noir ${NOIR_HOME}/noir wordcount_assoc -- ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt
# ./bench-multi.py $@ wordcount-re timely ${NOIR_HOME}/bench/timely timely-wordcount_assoc -- ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt
# ./bench-multi.py $@ wordcount-re flink ${NOIR_HOME}/bench/flink/WordCount wordCount-1.0.jar -- -input ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt

######./bench-multi.py $@ wordcount-windowed flink ${NOIR_HOME}/bench/flink/WordCountWindowed WordCountWindowed-0.1.jar -- -input ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt -win_size 10 -win_steps 5
######./bench-multi.py $@ wordcount-windowed mpi ${NOIR_HOME}/bench/mpi/windowed_wordcount_gc main_batched -- -d ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt
######./bench-multi.py $@ wordcount-windowed noir ${NOIR_HOME}/noir wordcount_windowed -- ${NOIR_HOME}/data/wordcount/gutenberg_4g.txt

# ./bench-multi.py $@ car-accidents-shared noir ${NOIR_HOME}/noir car_accidents -- ${NOIR_HOME}/data/accidents/NYPD_Motor_Vehicle_Collisions25.csv true
# ./bench-multi.py $@ car-accidents noir ${NOIR_HOME}/noir car_accidents -- ${NOIR_HOME}/data/accidents/NYPD_Motor_Vehicle_Collisions25.csv false
# ./bench-multi.py $@ car-accidents flink ${NOIR_HOME}/bench/flink/CarAccidents carAccidents-1.0.jar -- -input ${NOIR_HOME}/data/accidents/NYPD_Motor_Vehicle_Collisions25.csv
# ./bench-multi.py $@ car-accidents mpi ${NOIR_HOME}/bench/mpi/CarAccidents main -- -d ${NOIR_HOME}/data/accidents/NYPD_Motor_Vehicle_Collisions25.csv -T 1 -b

# ./bench-multi.py $@ enum-triangles mpi ${NOIR_HOME}/bench/mpi/EnumTriangles main -- -d ${NOIR_HOME}/data/triangles/1500-900k-42.csv
# ./bench-multi.py $@ enum-triangles flink ${NOIR_HOME}/bench/flink/EnumTriangles enumTriangles-1.0.jar -- -edges ${NOIR_HOME}/data/triangles/1500-900k-42.csv
# ./bench-multi.py $@ enum-triangles noir ${NOIR_HOME}/noir triangles_fold -- ${NOIR_HOME}/data/triangles/1500-900k-42.csv

# ./bench-multi.py $@ connected-components mpi ${NOIR_HOME}/bench/mpi/ConnectedComponents main -- -i 100 -n 200000 -d ${NOIR_HOME}/data/connected-components/edges.txt
# ./bench-multi.py $@ connected-components flink ${NOIR_HOME}/bench/flink/ConnectedComponents connectedComponents-1.0.jar -- -iterations 100 -vertices ${NOIR_HOME}/data/connected-components/nodes.txt -edges ${NOIR_HOME}/data/connected-components/edges.txt
# ./bench-multi.py $@ connected-components noir ${NOIR_HOME}/noir connected_components -- 100 200000 ${NOIR_HOME}/data/connected-components/nodes.txt ${NOIR_HOME}/data/connected-components/edges.txt

# ./bench-multi.py $@ pagerank mpi ${NOIR_HOME}/bench/mpi/PageRank main -- -i 100 -n 81306 -d ${NOIR_HOME}/data/pagerank/edges.txt
# ./bench-multi.py $@ pagerank flink ${NOIR_HOME}/bench/flink/PageRank pagerank-1.0.jar -- -iterations 100 -numPages 81306 -pages ${NOIR_HOME}/data/pagerank/nodes.txt -links ${NOIR_HOME}/data/pagerank/edges.txt
# ./bench-multi.py $@ pagerank noir ${NOIR_HOME}/noir pagerank_stateful -- 100 81306 ${NOIR_HOME}/data/pagerank/nodes.txt ${NOIR_HOME}/data/pagerank/edges.txt

# ./bench-multi.py $@ kmeans-30c-30it-200M mpi ${NOIR_HOME}/bench/mpi/KMeans main -- -cn 30 -it 30 -if ${NOIR_HOME}/data/kmeans/200M.csv -nw
# ./bench-multi.py $@ kmeans-300c-30it-200M mpi ${NOIR_HOME}/bench/mpi/KMeans main -- -cn 300 -it 30 -if ${NOIR_HOME}/data/kmeans/200M.csv -nw
# ./bench-multi.py $@ kmeans-30c-30it-2000M mpi ${NOIR_HOME}/bench/mpi/KMeans main -- -cn 30 -it 30 -if ${NOIR_HOME}/data/kmeans/2000M.csv -nw

# ./bench-multi.py $@ --alias noir-tls kmeans-30c-30it-200M noir ${NOIR_HOME}/noir kmeans -- 30 30 ${NOIR_HOME}/data/kmeans/200M.csv
# ./bench-multi.py $@ --alias noir-tls kmeans-300c-30it-200M noir ${NOIR_HOME}/noir kmeans -- 300 30 ${NOIR_HOME}/data/kmeans/200M.csv
# ./bench-multi.py $@ --alias noir-tls kmeans-30c-30it-2000M noir ${NOIR_HOME}/noir kmeans -- 30 30 ${NOIR_HOME}/data/kmeans/2000M.csv

# ./bench-multi.py $@ kmeans-30c-30it-200M flink ${NOIR_HOME}/bench/flink/KMeans kmeans-0.1.jar -- -n 30 -iter 30 -points ${NOIR_HOME}/data/kmeans/200M.csv
# ./bench-multi.py $@ kmeans-300c-30it-200M flink ${NOIR_HOME}/bench/flink/KMeans kmeans-0.1.jar -- -n 300 -iter 30 -points ${NOIR_HOME}/data/kmeans/200M.csv
# ./bench-multi.py $@ kmeans-30c-30it-2000M flink ${NOIR_HOME}/bench/flink/KMeans kmeans-0.1.jar -- -n 30 -iter 30 -points ${NOIR_HOME}/data/kmeans/2000M.csv

###### ./bench-multi.py $@ transitive-closure noir ${NOIR_HOME}/noir transitive_closure -- 1000 ${NOIR_HOME}/data/transitive-closure/transitive-2000-3000-4242.csv
###### ./bench-multi.py $@ transitive-closure flink ${NOIR_HOME}/bench/flink/TransitiveClosureNaive closure-1.0.jar -- -iterations 1000 -edges ${NOIR_HOME}/data/transitive-closure/transitive-2000-3000-4242.csv
