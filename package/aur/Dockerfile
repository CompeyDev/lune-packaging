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

COPY ~/ssh_creds/.ssh /home/builder/.ssh

COPY . /home/builder/packaging_scripts

RUN chown -R builder:builder /home/builder/.ssh && \
    chmod 600 /home/builder/.ssh/* -R && \
    chown -R builder:builder /home/builder/packaging_scripts && \
    chmod +x /home/builder/packaging_scripts/*.sh

USER builder
WORKDIR /home/builder/packaging_scripts

ENTRYPOINT ["/home/builder/packaging_scripts/publish.sh"]
