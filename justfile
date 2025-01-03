deploy server:
    rsync \
        --archive \
        --compress \
        --progress \
        --exclude-from=.deployignore \
        . \
        {{ server }}:~/yatesi
