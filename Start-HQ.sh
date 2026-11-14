#!/usr/bin/env sh
set -eu
cd "$(dirname "$0")"
export HQ_OPEN_BROWSER=1
if [ -x runtime/node ]; then exec runtime/node native-bridge/server.mjs; fi
exec node native-bridge/server.mjs
