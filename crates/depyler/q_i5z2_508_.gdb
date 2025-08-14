# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpHYDH9P/q_i5z2_508_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
