FROM jboss/wildfly:latest

ADD modifications /opt/jboss/wildfly/modifications/

ADD modules /opt/jboss/wildfly/modules/

RUN /opt/jboss/wildfly/modifications/modify-standalone.sh > /tmp/build-info.txt

ADD app.war /opt/jboss/wildfly/standalone/deployments/

