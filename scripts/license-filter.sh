#!/bin/sh

HEADER='// SPDX-License-Identifier: WTFPL'

case "$1" in
    clean)
        sed "/^\/\/ SPDX-License-Identifier: WTFPL$/d" |
            sed "1i\\
$HEADER
"
        ;;

    smudge)
        sed "/^\/\/ SPDX-License-Identifier: WTFPL$/d"
        ;;

    *)
        echo "Usage: $0 {clean|smudge}" >&2
        exit 1
        ;;
esac
