#!/usr/bin/env bash



export LOG_DIR=/opt/edp/monitor1/log
export PID_DIR=/opt/edp/monitor1/data/alertmanager

export HOSTNAME=`hostname`

log=$LOG_DIR/alertmanager-$HOSTNAME.out
pid=$PID_DIR/alertmanager.pid

echo "========================start alertmanager========================"

exec_command="alertmanager --config.file=/opt/edp/monitor1/conf/alertmanager.yml --storage.path="/opt/edp/monitor1/data/alertmanager" --cluster.advertise-address=0.0.0.0:9093 "
echo "nohup $exec_command > $log 2>&1 &"
nohup $exec_command > $log 2>&1 &
echo $! > $pid

tail -f /dev/null
