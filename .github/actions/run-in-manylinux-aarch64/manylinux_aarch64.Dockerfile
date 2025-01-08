FROM quay.io/pypa/manylinux2014_aarch64
COPY entry.sh /entry.sh
ENTRYPOINT ["bash", "/entry.sh"]
