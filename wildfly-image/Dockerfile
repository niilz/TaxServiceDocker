FROM gradle:latest AS build
# TODO: get sources from github or something
ADD ./taxservice/ /home/gradle/taxservice/
WORKDIR /home/gradle/taxservice/
RUN gradle clean build | tee /tmp/gradle-build.txt
#COPY app.war .
CMD ["bash"]

FROM jboss/wildfly:latest
WORKDIR /opt/jboss/wildfly/
ADD modifications /opt/jboss/wildfly/modifications/
ADD modules /opt/jboss/wildfly/modules/
RUN /opt/jboss/wildfly/modifications/modify-standalone.sh > /tmp/wildfly-build.txt
COPY --from=build /home/gradle/taxservice/app/build/libs/app.war /opt/jboss/wildfly/standalone/deployments/
