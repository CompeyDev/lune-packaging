FROM archlinux:base-devel

RUN pacman --needed --noconfirm -Syu \
    base \
    git \
    pacman-contrib \
    cargo
    
RUN useradd -m builder && \
    echo "builder ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    echo "%wheel ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    usermod -a -G wheel builder

RUN mkdir -p /home/builder/.ssh && \
    touch /home/builder/.ssh/known_hosts

COPY lune-bin/ /home/builder/packages/lune-bin
COPY lune-git/ /home/builder/packages/lune-git
COPY lune/ /home/builder/packages/lune

COPY ./entrypoint.sh /home/builder/packages/entrypoint.sh

RUN chown -R builder:builder /home/builder/packages && \
    chmod 700 /home/builder/packages/* -R && \
    # chown -R builder:builder /home/builder/packages/lune-bin && \
    # chmod 600 /home/builder/packages/lune-bin -R && \
    # chown -R builder:builder /home/builder/packages/lune-git && \
    # chmod 600 /home/builder/packages/lune-git -R && \
    # chown -R builder:builder /home/builder/packages/lune && \
    # chmod 600 /home/builder/packages/lune -R && \
    chmod +x /home/builder/packages/*.sh

USER builder
WORKDIR /home/builder/packages

RUN ls /home/builder/packages

ENTRYPOINT ["/home/builder/packages/entrypoint.sh"]
