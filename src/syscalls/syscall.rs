// This file is based on https://github.com/iovisor/bcc/blob/222821c8be2c3aa862ddd7e4bf2a10965ae0639f/src/python/bcc/syscall.py
//
// Copyright 2017 Sasha Goldshtein
// Copyright 2018 Red Hat, Inc.

// Syscall table for Linux x86_64, not very recent.
// generated from strace/linux/x86_64/syscallent.h

pub fn syscall_name(syscall_num: u32) -> Option<&'static str> {
    match syscall_num {
        0 => Some("read"),
        1 => Some("write"),
        2 => Some("open"),
        3 => Some("close"),
        4 => Some("stat"),
        5 => Some("fstat"),
        6 => Some("lstat"),
        7 => Some("poll"),
        8 => Some("lseek"),
        9 => Some("mmap"),
        10 => Some("mprotect"),
        11 => Some("munmap"),
        12 => Some("brk"),
        13 => Some("rt_sigaction"),
        14 => Some("rt_sigprocmask"),
        15 => Some("rt_sigreturn"),
        16 => Some("ioctl"),
        17 => Some("pread64"),
        18 => Some("pwrite64"),
        19 => Some("readv"),
        20 => Some("writev"),
        21 => Some("access"),
        22 => Some("pipe"),
        23 => Some("select"),
        24 => Some("sched_yield"),
        25 => Some("mremap"),
        26 => Some("msync"),
        27 => Some("mincore"),
        28 => Some("madvise"),
        29 => Some("shmget"),
        30 => Some("shmat"),
        31 => Some("shmctl"),
        32 => Some("dup"),
        33 => Some("dup2"),
        34 => Some("pause"),
        35 => Some("nanosleep"),
        36 => Some("getitimer"),
        37 => Some("alarm"),
        38 => Some("setitimer"),
        39 => Some("getpid"),
        40 => Some("sendfile"),
        41 => Some("socket"),
        42 => Some("connect"),
        43 => Some("accept"),
        44 => Some("sendto"),
        45 => Some("recvfrom"),
        46 => Some("sendmsg"),
        47 => Some("recvmsg"),
        48 => Some("shutdown"),
        49 => Some("bind"),
        50 => Some("listen"),
        51 => Some("getsockname"),
        52 => Some("getpeername"),
        53 => Some("socketpair"),
        54 => Some("setsockopt"),
        55 => Some("getsockopt"),
        56 => Some("clone"),
        57 => Some("fork"),
        58 => Some("vfork"),
        59 => Some("execve"),
        60 => Some("exit"),
        61 => Some("wait4"),
        62 => Some("kill"),
        63 => Some("uname"),
        64 => Some("semget"),
        65 => Some("semop"),
        66 => Some("semctl"),
        67 => Some("shmdt"),
        68 => Some("msgget"),
        69 => Some("msgsnd"),
        70 => Some("msgrcv"),
        71 => Some("msgctl"),
        72 => Some("fcntl"),
        73 => Some("flock"),
        74 => Some("fsync"),
        75 => Some("fdatasync"),
        76 => Some("truncate"),
        77 => Some("ftruncate"),
        78 => Some("getdents"),
        79 => Some("getcwd"),
        80 => Some("chdir"),
        81 => Some("fchdir"),
        82 => Some("rename"),
        83 => Some("mkdir"),
        84 => Some("rmdir"),
        85 => Some("creat"),
        86 => Some("link"),
        87 => Some("unlink"),
        88 => Some("symlink"),
        89 => Some("readlink"),
        90 => Some("chmod"),
        91 => Some("fchmod"),
        92 => Some("chown"),
        93 => Some("fchown"),
        94 => Some("lchown"),
        95 => Some("umask"),
        96 => Some("gettimeofday"),
        97 => Some("getrlimit"),
        98 => Some("getrusage"),
        99 => Some("sysinfo"),
        100 => Some("times"),
        101 => Some("ptrace"),
        102 => Some("getuid"),
        103 => Some("syslog"),
        104 => Some("getgid"),
        105 => Some("setuid"),
        106 => Some("setgid"),
        107 => Some("geteuid"),
        108 => Some("getegid"),
        109 => Some("setpgid"),
        110 => Some("getppid"),
        111 => Some("getpgrp"),
        112 => Some("setsid"),
        113 => Some("setreuid"),
        114 => Some("setregid"),
        115 => Some("getgroups"),
        116 => Some("setgroups"),
        117 => Some("setresuid"),
        118 => Some("getresuid"),
        119 => Some("setresgid"),
        120 => Some("getresgid"),
        121 => Some("getpgid"),
        122 => Some("setfsuid"),
        123 => Some("setfsgid"),
        124 => Some("getsid"),
        125 => Some("capget"),
        126 => Some("capset"),
        127 => Some("rt_sigpending"),
        128 => Some("rt_sigtimedwait"),
        129 => Some("rt_sigqueueinfo"),
        130 => Some("rt_sigsuspend"),
        131 => Some("sigaltstack"),
        132 => Some("utime"),
        133 => Some("mknod"),
        134 => Some("useli"),
        135 => Some("personality"),
        136 => Some("ustat"),
        137 => Some("statfs"),
        138 => Some("fstatfs"),
        139 => Some("sysfs"),
        140 => Some("getpriority"),
        141 => Some("setpriority"),
        142 => Some("sched_setparam"),
        143 => Some("sched_getparam"),
        144 => Some("sched_setscheduler"),
        145 => Some("sched_getscheduler"),
        146 => Some("sched_get_priority_max"),
        147 => Some("sched_get_priority_min"),
        148 => Some("sched_rr_get_interval"),
        149 => Some("mlock"),
        150 => Some("munlock"),
        151 => Some("mlockall"),
        152 => Some("munlockall"),
        153 => Some("vhangup"),
        154 => Some("modify_ldt"),
        155 => Some("pivot_root"),
        156 => Some("_sysctl"),
        157 => Some("prctl"),
        158 => Some("arch_prctl"),
        159 => Some("adjtimex"),
        160 => Some("setrlimit"),
        161 => Some("chroot"),
        162 => Some("sync"),
        163 => Some("acct"),
        164 => Some("settimeofday"),
        165 => Some("mount"),
        166 => Some("umount2"),
        167 => Some("swapon"),
        168 => Some("swapoff"),
        169 => Some("reboot"),
        170 => Some("sethostname"),
        171 => Some("setdomainname"),
        172 => Some("iopl"),
        173 => Some("ioperm"),
        174 => Some("create_module"),
        175 => Some("init_module"),
        176 => Some("delete_module"),
        177 => Some("get_kernel_syms"),
        178 => Some("query_module"),
        179 => Some("quotactl"),
        180 => Some("nfsservctl"),
        181 => Some("getpmsg"),
        182 => Some("putpmsg"),
        183 => Some("afs_syscall"),
        184 => Some("tuxcall"),
        185 => Some("security"),
        186 => Some("gettid"),
        187 => Some("readahead"),
        188 => Some("setxattr"),
        189 => Some("lsetxattr"),
        190 => Some("fsetxattr"),
        191 => Some("getxattr"),
        192 => Some("lgetxattr"),
        193 => Some("fgetxattr"),
        194 => Some("listxattr"),
        195 => Some("llistxattr"),
        196 => Some("flistxattr"),
        197 => Some("removexattr"),
        198 => Some("lremovexattr"),
        199 => Some("fremovexattr"),
        200 => Some("tkill"),
        201 => Some("time"),
        202 => Some("futex"),
        203 => Some("sched_setaffinity"),
        204 => Some("sched_getaffinity"),
        205 => Some("set_thread_area"),
        206 => Some("io_setup"),
        207 => Some("io_destroy"),
        208 => Some("io_getevents"),
        209 => Some("io_submit"),
        210 => Some("io_cancel"),
        211 => Some("get_thread_area"),
        212 => Some("lookup_dcookie"),
        213 => Some("epoll_create"),
        214 => Some("epoll_ctl_old"),
        215 => Some("epoll_wait_old"),
        216 => Some("remap_file_pages"),
        217 => Some("getdents64"),
        218 => Some("set_tid_address"),
        219 => Some("restart_syscall"),
        220 => Some("semtimedop"),
        221 => Some("fadvise64"),
        222 => Some("timer_create"),
        223 => Some("timer_settime"),
        224 => Some("timer_gettime"),
        225 => Some("timer_getoverrun"),
        226 => Some("timer_delete"),
        227 => Some("clock_settime"),
        228 => Some("clock_gettime"),
        229 => Some("clock_getres"),
        230 => Some("clock_nanosleep"),
        231 => Some("exit_group"),
        232 => Some("epoll_wait"),
        233 => Some("epoll_ctl"),
        234 => Some("tgkill"),
        235 => Some("utimes"),
        236 => Some("vserver"),
        237 => Some("mbind"),
        238 => Some("set_mempolicy"),
        239 => Some("get_mempolicy"),
        240 => Some("mq_open"),
        241 => Some("mq_unlink"),
        242 => Some("mq_timedsend"),
        243 => Some("mq_timedreceive"),
        244 => Some("mq_notify"),
        245 => Some("mq_getsetattr"),
        246 => Some("kexec_load"),
        247 => Some("waitid"),
        248 => Some("add_key"),
        249 => Some("request_key"),
        250 => Some("keyctl"),
        251 => Some("ioprio_set"),
        252 => Some("ioprio_get"),
        253 => Some("inotify_init"),
        254 => Some("inotify_add_watch"),
        255 => Some("inotify_rm_watch"),
        256 => Some("migrate_pages"),
        257 => Some("openat"),
        258 => Some("mkdirat"),
        259 => Some("mknodat"),
        260 => Some("fchownat"),
        261 => Some("futimesat"),
        262 => Some("newfstatat"),
        263 => Some("unlinkat"),
        264 => Some("renameat"),
        265 => Some("linkat"),
        266 => Some("symlinkat"),
        267 => Some("readlinkat"),
        268 => Some("fchmodat"),
        269 => Some("faccessat"),
        270 => Some("pselect6"),
        271 => Some("ppoll"),
        272 => Some("unshare"),
        273 => Some("set_robust_list"),
        274 => Some("get_robust_list"),
        275 => Some("splice"),
        276 => Some("tee"),
        277 => Some("sync_file_range"),
        278 => Some("vmsplice"),
        279 => Some("move_pages"),
        280 => Some("utimensat"),
        281 => Some("epoll_pwait"),
        282 => Some("signalfd"),
        283 => Some("timerfd_create"),
        284 => Some("eventfd"),
        285 => Some("fallocate"),
        286 => Some("timerfd_settime"),
        287 => Some("timerfd_gettime"),
        288 => Some("accept4"),
        289 => Some("signalfd4"),
        290 => Some("eventfd2"),
        291 => Some("epoll_create1"),
        292 => Some("dup3"),
        293 => Some("pipe2"),
        294 => Some("inotify_init1"),
        295 => Some("preadv"),
        296 => Some("pwritev"),
        297 => Some("rt_tgsigqueueinfo"),
        298 => Some("perf_event_open"),
        299 => Some("recvmmsg"),
        300 => Some("fanotify_init"),
        301 => Some("fanotify_mark"),
        302 => Some("prlimit64"),
        303 => Some("name_to_handle_at"),
        304 => Some("open_by_handle_at"),
        305 => Some("clock_adjtime"),
        306 => Some("syncfs"),
        307 => Some("sendmmsg"),
        308 => Some("setns"),
        309 => Some("getcpu"),
        310 => Some("process_vm_readv"),
        311 => Some("process_vm_writev"),
        312 => Some("kcmp"),
        313 => Some("finit_module"),
        314 => Some("sched_setattr"),
        315 => Some("sched_getattr"),
        316 => Some("renameat2"),
        317 => Some("seccomp"),
        318 => Some("getrandom"),
        319 => Some("memfd_create"),
        320 => Some("kexec_file_load"),
        321 => Some("bpf"),
        322 => Some("execveat"),
        323 => Some("userfaultfd"),
        324 => Some("membarrier"),
        325 => Some("mlock2"),
        326 => Some("copy_file_range"),
        327 => Some("preadv2"),
        328 => Some("pwritev2"),
        329 => Some("pkey_mprotect"),
        330 => Some("pkey_alloc"),
        331 => Some("pkey_free"),
        332 => Some("statx"),
        333 => Some("io_pgetevents"),
        334 => Some("rseq"),
        _ => None,
    }
}
