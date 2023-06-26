#!/usr/bin/env bash

source /opt/edp/hdfs1/conf/hadoop-hdfs-env.sh

echo "========================starting journalnode========================"
${HADOOP_HOME}/sbin/hadoop-daemon.sh --config /opt/edp/hdfs1/conf start journalnode

tail -f /dev/null
