#!/usr/bin/env bash



export LOG_DIR=/opt/edp/monitor1/log
export PID_DIR=/opt/edp/monitor1/data/prometheus

export HOSTNAME=`hostname`

log=$LOG_DIR/prometheus-$HOSTNAME.out
pid=$PID_DIR/prometheus.pid

echo "========================start prometheus========================"

exec_command="prometheus --config.file=/opt/edp/monitor1/conf/prometheus.yml --storage.tsdb.path="/opt/edp/monitor1/data/prometheus"  --web.listen-address=0.0.0.0:9090 --web.enable-lifecycle"
echo "nohup $exec_command > $log 2>&1 &"
nohup $exec_command > $log 2>&1 &
echo $! > $pid

tail -f /dev/null
