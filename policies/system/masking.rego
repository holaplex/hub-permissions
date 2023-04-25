package system.log

# remove tokens and cookies from decision_logs
mask["/input/request/headers/authorization"]

mask["/input/request/headers/cookie"]
