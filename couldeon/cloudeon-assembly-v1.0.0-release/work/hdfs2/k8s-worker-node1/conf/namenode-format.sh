#!/usr/bin/env bash

set -o errexit
set -o errtrace
set -o pipefail
set -o xtrace

source /opt/edp/hdfs2/conf/hadoop-hdfs-env.sh
_METADATA_DIR=/opt/edp/hdfs2/data/namenode/current





 if [[ ! -d $_METADATA_DIR ]]; then
     echo "无法检测到namenode元数据文件夹，开始进行namenode格式化。。。。。。。。。。"
     yes Y|  hdfs --config /opt/edp/hdfs2/conf namenode -format hdfscluster1
 fi

