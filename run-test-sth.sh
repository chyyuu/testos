bootimage run --bin test-something --target x86_64-blog_os.json -- -serial mon:stdio -display none \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04
