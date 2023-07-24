FROM archlinux:base-devel

RUN pacman --needed --noconfirm -Syu \
    base \
    git \
    pacman-contrib \
    openssh
    
RUN useradd -m builder && \
    echo "builder ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    echo "%wheel ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    usermod -a -G wheel builder

RUN mkdir -p /home/builder/.ssh && \
    touch /home/builder/.ssh/known_hosts

COPY . /home/builder/packaging_scripts

RUN mv /home/builder/packaging_scripts/ssh_config /home/builder/.ssh/config

RUN chown -R builder:builder /home/builder/.ssh && \
    chmod 600 /home/builder/.ssh/* -R && \
    chown -R builder:builder /home/builder/packaging_scripts

USER builder
WORKDIR /home/builder

ENTRYPOINT ["/home/builder/packaging_scripts/publish.sh"]
