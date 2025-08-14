# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpYRakjq/lsk6I2_s6p8Tgy2wK0Pu2.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
