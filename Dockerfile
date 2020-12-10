FROM registry.fedoraproject.org/fedora-minimal:33

LABEL org.opencontainers.image.source="https://github.com/drogue-iot/drogue-bluetooth-gateway"

RUN microdnf install -y dbus-devel

ADD target/release/drogue-bluetooth-gateway /

ENTRYPOINT [ "/drogue-bluetooth-gateway" ]
