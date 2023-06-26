#!/usr/bin/env bash

source /opt/edp/hdfs2/conf/hadoop-hdfs-env.sh

echo "========================starting httpfs========================"
${HADOOP_HOME}/sbin/hadoop-daemon.sh --config /opt/edp/hdfs2/conf start httpfs


tail -f /dev/null
