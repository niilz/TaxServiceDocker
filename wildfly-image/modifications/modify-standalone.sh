#!/bin/bash

WILDFLY_HOME=/opt/jboss/wildfly
JBOSS_CLI=$WILDFLY_HOME/bin/jboss-cli.sh
WILDFLY_CONFIG="standalone.xml"

function wait_for_server() {
	until `$JBOSS_CLI -c "ls /deployment" &> /dev/null`; do
		sleep 1
	done
}

echo "=> Starting wildfly"
$WILDFLY_HOME/bin/standalone.sh -c $WILDFLY_CONFIG > /dev/null &

echo "=> Waiting for wildfly to boot.."
wait_for_server

echo "applying modifications to standalone.xml"
$JBOSS_CLI -c --file=`dirname "$0"`/modifications.cli

echo "=> Shutting down wildfly"
$JBOSS_CLI -c ":shutdown"
