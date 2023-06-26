#!/usr/bin/env bash

source /opt/edp/hdfs1/conf/hadoop-hdfs-env.sh

echo "========================starting httpfs========================"
${HADOOP_HOME}/sbin/hadoop-daemon.sh --config /opt/edp/hdfs1/conf start httpfs


tail -f /dev/null
