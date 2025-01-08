FROM quay.io/pypa/manylinux2014_x86_64
COPY entry.sh /entry.sh
ENTRYPOINT ["bash", "/entry.sh"]
