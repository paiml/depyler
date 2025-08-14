# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpc0A5VY/eSNZP9FaJ_a0_sQ.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
