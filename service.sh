#!/system/bin/sh

MODDIR=${0%/*}

resetprop -w sys.boot_completed 0

sleep 5

chmod +x $MODDIR/SCM

"$MODDIR/SCM" &