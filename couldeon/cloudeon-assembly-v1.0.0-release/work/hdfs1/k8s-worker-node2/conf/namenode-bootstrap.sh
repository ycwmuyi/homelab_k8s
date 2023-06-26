#!/usr/bin/env bash



source /opt/edp/hdfs1/conf/hadoop-hdfs-env.sh


echo "========================start zkfc========================"
${HADOOP_HOME}/sbin/hadoop-daemon.sh --config /opt/edp/hdfs1/conf start zkfc
echo "========================start namenode========================"
${HADOOP_HOME}/sbin/hadoop-daemon.sh --config /opt/edp/hdfs1/conf start namenode

tail -f /dev/null
