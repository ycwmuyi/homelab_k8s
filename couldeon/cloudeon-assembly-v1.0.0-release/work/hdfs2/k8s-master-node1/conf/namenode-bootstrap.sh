#!/usr/bin/env bash



source /opt/edp/hdfs2/conf/hadoop-hdfs-env.sh


echo "========================start zkfc========================"
${HADOOP_HOME}/sbin/hadoop-daemon.sh --config /opt/edp/hdfs2/conf start zkfc
echo "========================start namenode========================"
${HADOOP_HOME}/sbin/hadoop-daemon.sh --config /opt/edp/hdfs2/conf start namenode

tail -f /dev/null
