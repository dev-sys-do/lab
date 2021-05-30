# download and unpack an ubuntu image
skopeo copy docker://ubuntu:xenial oci:ubuntu:xenial
sudo umoci unpack --image ubuntu:latest bundle

# Run a container

# Prepare the rootfs
mount -o bind $PWD/bundle/rootfs/proc $PWD/bundle/rootfs/proc
mount -o ro,bind /etc/resolv.conf $PWD/bundle/rootfs/etc/resolv.conf
mount -o ro,bind /dev $PWD/bundle/rootfs/dev

# Basic i.e. chroot
unshare --fork chroot $PWD/bundle/rootfs /bin/bash

# PID
unshare --pid --fork chroot $PWD/bundle/rootfs /bin/bash
echo $$

# PID + mount namespace
unshare --pid --fork --mount --mount-proc=$PWD/bundle/rootfs/proc chroot $PWD/bundle/rootfs /bin/bash

# PID + mount + IPC namespace
unshare --pid --mount --mount-proc=$PWD/bundle/rootfs/proc --ipc --fork chroot $PWD/bundle/rootfs /bin/bash

# PID + mount + IPC + networking namespace
mkdir -p /var/foo/namespaces
mount --bind /var/foo/namespaces/ /var/foo/namespaces/
touch /var/foo/namespaces/netns-1
unshare --pid --mount --mount-proc=$PWD/bundle/rootfs/proc --ipc --net=/var/foo/namespaces/netns-1 --fork chroot $PWD/bundle/rootfs /bin/bash

# Place the container in a cgroup

# Create a cpuset cgroup
mkdir /sys/fs/cgroup/cpuset/foo
echo 0-2 > /sys/fs/cgroup/cpuset/foo/cpuset.cpus
echo 0 > /sys/fs/cgroup/cpuset/foo/cpuset.mems

# Create a cpu share cgroup
mkdir /sys/fs/cgroup/cpu/foo
echo 100 > /sys/fs/cgroup/cpu/foo/cpu.shares

# Move the container process into the cpugroups
echo <unshare_pid> > /sys/fs/cgroup/cpu/foo/tasks
echo <unshare_pid> > /sys/fs/cgroup/cpuset/foo/tasks
