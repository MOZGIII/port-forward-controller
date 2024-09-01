if os.path.exists('./Tiltfile.local'):
  include('./Tiltfile.local')

docker_build('ghcr.io/mozgiii/port-forward-controller', '.')
k8s_yaml(helm('charts/port-forward-controller'))


k8s_resource(
  new_name = 'deployment',
  workload='chart-port-forward-controller',
  labels=["chart"],
)

k8s_resource(
  new_name = 'crd',
  objects=[
    'pcpmaps.port-forward.io:customresourcedefinition',
  ],
  labels=["chart"],
)

k8s_resource(
  new_name = 'rbac',
  objects=[
    'chart-port-forward-controller:serviceaccount',
    'chart-port-forward-controller:clusterrole',
    'chart-port-forward-controller:clusterrolebinding',
  ],
  labels=["chart"],
)
