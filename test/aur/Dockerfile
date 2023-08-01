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

COPY lune-bin/ lune-git/ lune/ /home/builder/packages/

COPY ./entrypoint.sh /home/builder/packages/entrypoint.sh

RUN chown -R builder:builder /home/builder/packages && \
    chmod 600 /home/builder/packages/* -R

USER builder
WORKDIR /home/builder/packages

ENTRYPOINT ["/home/builder/packages/entrypoint.sh"]
