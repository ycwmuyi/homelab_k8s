#!/usr/bin/env bash

set -o errexit
set -o errtrace
set -o pipefail
set -o xtrace

source /opt/edp/hdfs2/conf/hadoop-hdfs-env.sh
_METADATA_DIR=/opt/edp/hdfs2/data/namenode/current





 if [[ ! -d $_METADATA_DIR ]]; then
   echo "检测到没有namenode的元数据文件夹，开始进行namenode的初始化操作，从checkpoint中加载。。。。。。。"
   yes Y|   hdfs --config /opt/edp/hdfs2/conf namenode -bootstrapStandby
 fi

