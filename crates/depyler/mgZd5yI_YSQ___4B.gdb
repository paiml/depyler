# GDB initialization script for Depyler debugging
# Source: /tmp/.tmphYqgkh/mgZd5yI_YSQ___4B.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
