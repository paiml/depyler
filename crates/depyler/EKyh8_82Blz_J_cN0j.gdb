# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpXmurfp/EKyh8_82Blz_J_cN0j.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
