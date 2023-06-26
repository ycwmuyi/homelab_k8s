#!/usr/bin/env bash

export ZOOKEEPER_LOG_DIR=/opt/edp/zookeeper1/log
export ZOOKEEPER_DATA_DIR=/opt/edp/zookeeper1/data
export ZOOPIDFILE="/opt/edp/zookeeper1/data/zookeeper-server.pid"

export SERVER_JVMFLAGS="-Dcom.sun.management.jmxremote.port=9911 -Dcom.sun.management.jmxremote.authenticate=false -Dcom.sun.management.jmxremote.ssl=false -Dcom.sun.management.jmxremote -Dcom.sun.management.jmxremote.local.only=false"

export SERVER_JVMFLAGS="-Xmx1024m $SERVER_JVMFLAGS"

export SERVER_JVMFLAGS="-Dzookeeper.log.dir=/opt/edp/zookeeper1/log -Dzookeeper.root.logger=INFO,ROLLINGFILE $SERVER_JVMFLAGS"

export SERVER_JVMFLAGS="-Dznode.container.checkIntervalMs=1000 $SERVER_JVMFLAGS"

export SERVER_JVMFLAGS=" -javaagent:/opt/jmx_exporter/jmx_prometheus_javaagent-0.14.0.jar=5541:/opt/edp/zookeeper1/conf/jmx_zookeeper.yaml $SERVER_JVMFLAGS"
