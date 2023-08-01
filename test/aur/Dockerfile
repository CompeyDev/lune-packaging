FROM archlinux:base-devel

RUN pacman --needed --noconfirm -Syu \
    base \
    git \
    pacman-contrib 
    
RUN useradd -m builder && \
    echo "builder ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    echo "%wheel ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    usermod -a -G wheel builder

RUN mkdir -p /home/builder/.ssh && \
    touch /home/builder/.ssh/known_hosts

COPY ../../package/aur/lune-bin ../../package/aur/lune-git ../../package/aur/lune /home/builder/packages

COPY ./entrypoint.sh /home/builder/packages/entrypoint.sh

RUN chown -R builder:builder /home/builder/.ssh && \
    chmod 600 /home/builder/.ssh/* -R && \
    chown -R builder:builder /home/builder/packages && \
    chmod 600 /home/builder/packages/* -R && \
    chmod +x /home/builder/packages/*.ssh

USER builder
WORKDIR /home/builder/packages

ENTRYPOINT ["/home/builder/packages/entrypoint.sh"]
