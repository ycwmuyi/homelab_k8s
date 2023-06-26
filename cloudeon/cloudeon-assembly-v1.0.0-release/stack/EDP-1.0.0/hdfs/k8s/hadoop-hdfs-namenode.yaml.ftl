---
apiVersion: "apps/v1"
kind: "Deployment"
metadata:
  labels:
    name: "${roleServiceFullName}"
  name: "${roleServiceFullName}"
  namespace: "cloudeon"
spec:
  replicas: ${roleNodeCnt}
  selector:
    matchLabels:
      app: "${roleServiceFullName}"
  strategy:
    type: "RollingUpdate"
    rollingUpdate:
      maxSurge: 0
      maxUnavailable: 1
  minReadySeconds: 5
  revisionHistoryLimit: 10
  template:
    metadata:
      labels:
        name: "${roleServiceFullName}"
        app: "${roleServiceFullName}"
        podConflictName: "${roleServiceFullName}"
      annotations:
        serviceInstanceName: "${service.serviceName}"
    spec:
      affinity:
        podAntiAffinity:
          requiredDuringSchedulingIgnoredDuringExecution:
          - labelSelector:
              matchLabels:
                name: "${roleServiceFullName}"
                podConflictName: "${roleServiceFullName}"
            namespaces:
            - "default"
            topologyKey: "kubernetes.io/hostname"
      hostPID: false
      hostNetwork: true
      initContainers:
        - name: namenode-format
          image: "${dockerImage}"
          args:
            - "/opt/edp/${service.serviceName}/conf/namenode-format.sh"
          volumeMounts:
            - mountPath: "/opt/edp/${service.serviceName}/data"
              name: "data"
            - mountPath: "/opt/edp/${service.serviceName}/log"
              name: "log"
            - mountPath: "/etc/localtime"
              name: "timezone"
            - mountPath: "/opt/edp/${service.serviceName}/conf"
              name: "conf"
      containers:
      - args:
        - "/opt/edp/${service.serviceName}/conf/namenode-bootstrap.sh"
        image: "${dockerImage}"
        imagePullPolicy: "Always"
        name: "${roleServiceFullName}"
        readinessProbe:
          httpGet:
            path: "/jmx?qry=Hadoop:service=NameNode,name=NameNodeInfo&&user.name=hdfs"
            port: ${conf['namenode.http-port']}
            scheme: "HTTP"
          failureThreshold: 3
          initialDelaySeconds: 10
          periodSeconds: 10
          successThreshold: 1
          timeoutSeconds: 1
        resources:
          requests: {}
          limits: {}
        securityContext:
          privileged: true
        volumeMounts:
        - mountPath: "/opt/edp/${service.serviceName}/data"
          name: "data"
        - mountPath: "/opt/edp/${service.serviceName}/log"
          name: "log"
        - mountPath: "/etc/localtime"
          name: "timezone"
        - mountPath: "/opt/edp/${service.serviceName}/conf"
          name: "conf"

      nodeSelector:
        ${roleServiceFullName}: "true"
      terminationGracePeriodSeconds: 30
      volumes:
      - hostPath:
          path: "/nfs/data/edp/${service.serviceName}/data"
        name: "data"
      - hostPath:
          path: "/nfs/data/edp/${service.serviceName}/log"
        name: "log"
      - hostPath:
          path: "/etc/localtime"
        name: "timezone"
      - hostPath:
          path: "/nfs/data/edp/${service.serviceName}/conf"
        name: "conf"

